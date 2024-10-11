use crate::cli::knowledge::knowledge_args::InitArgs;
use crate::config;
use crate::config::{get_knowledge_dir, Config};
use crate::context::{load_files_into_context, ContextConsumer};
use crate::knowledge::{EmbeddingServiceBuilder, EmbeddingServiceHandle};
use crate::persona::resolve_persona;
use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::task::JoinHandle;

/// Bounded channel size as a constant for readability.
const CHANNEL_SIZE: usize = 10;
const MAX_THREADS: usize = 10; // Maximum number of threads

/// Job type alias for clarity
type Job = (String, String);

/// Embedding data stored in the database with file name and calculated embedding.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub file_name: String,
    pub embedding: Vec<f32>, // Adjust size based on the model
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/// Fetch configuration settings as a clone.
fn get_config() -> Config {
    let config = config::CONFIG.lock().unwrap();
    config.clone()
}

/// Entry point for initializing the knowledge system.
pub async fn init(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let db = initialize_database().await?;
    let (sender, receiver) = async_channel::bounded(CHANNEL_SIZE);
    let persona = resolve_persona(&args.persona, config.default_persona.as_str())?;
    let embedding_client = create_embedding_client(args.model, &config)?;

    // Spawn workers for processing jobs.
    let handles = spawn_workers(receiver, db, embedding_client);

    // Load and process files
    process_files(sender, &persona)?;

    // Close the channel and wait for workers to finish.
    await_worker_completion(handles).await;

    Ok(())
}

/// Initialize the database connection and switch to the correct namespace/database.
async fn initialize_database() -> Result<Surreal<Db>, Box<dyn Error>> {
    let db = Surreal::new::<RocksDb>(get_knowledge_dir()?.to_str().unwrap()).await?;
    db.use_ns("knowledge").use_db("knowledge_db").await?;
    Ok(db)
}

/// Create the appropriate embedding client based on user input or configuration.
fn create_embedding_client(
    model: Option<String>,
    config: &Config,
) -> Result<EmbeddingServiceHandle, Box<dyn Error>> {
    let model_name = model.unwrap_or_else(|| config.ai.embedding_model.clone());
    let embedding_client = EmbeddingServiceBuilder::new()
        .model_name(model_name.into())
        .build()?;
    Ok(embedding_client)
}

/// Spawn workers that will handle processing jobs concurrently.
fn spawn_workers(
    receiver: Receiver<Job>,
    db: Surreal<Db>,
    client: EmbeddingServiceHandle,
) -> Vec<JoinHandle<()>> {
    (0..MAX_THREADS)
        .map(|_| {
            let receiver_clone = receiver.clone();
            let db_clone = db.clone();
            let client_clone = client.clone();

            tokio::spawn(async move {
                process_jobs(receiver_clone, db_clone, client_clone).await;
            })
        })
        .collect()
}

/// Process jobs received from the channel by retrieving embeddings and saving them to the database.
async fn process_jobs(receiver: Receiver<Job>, db: Surreal<Db>, client: EmbeddingServiceHandle) {
    while let Ok((filename, content)) = receiver.recv().await {
        eprintln!("Processing File: {}", filename);

        // Now handle success or failure from embedding processing
        let embedding = match client.inner.get_embedding(content.clone()).await {
            Ok(embedding) => {
                // Assemble the embedding data (once get_embedding has been successful)
                Some(EmbeddingData {
                    file_name: filename.clone(),
                    embedding: embedding.to_vec(),
                    metadata: Some("Additional details if needed".to_string()),
                })
            }
            Err(err) => {
                // Handle the error from the embedding service
                eprintln!("Failed to get embedding for {}: {}", filename, err);
                None
            }
        };
        if let Some(embedding) = embedding {
            // At this point, the embedding has been processed; now we can make the db call.

            if let Err(e) = db
                .upsert::<Option<Record>>(("context_embeddings", filename.clone()))
                .content(embedding)
                .await
            {
                eprintln!("Failed to store embedding for {} {}", filename, e);
            }
        }
    }
}
/// Loads files into the knowledge context, ready for processing.
fn process_files(
    sender: Sender<Job>,
    persona: &crate::persona::Persona,
) -> Result<(), Box<dyn Error>> {
    let mut consumer = Consumer { sender };
    load_files_into_context(&mut consumer, &env::current_dir()?, &persona.file_types)?;
    // Close the sender when done processing files.
    drop(consumer);
    Ok(())
}

/// Wait for all worker threads to finish processing.
async fn await_worker_completion(handles: Vec<JoinHandle<()>>) {
    for handle in handles {
        if let Err(err) = handle.await {
            eprintln!("Error in worker thread: {:?}", err);
        }
    }
}

/// Consumer implementation that sends jobs to a channel for asynchronous processing.
struct Consumer {
    sender: Sender<Job>,
}

impl ContextConsumer for Consumer {
    fn consume(&mut self, filename: Cow<str>, content: Cow<str>) -> Result<(), Box<dyn Error>> {
        let job = (filename.into(), content.into());
        tokio::task::block_in_place(|| self.sender.send_blocking(job))?;
        Ok(())
    }
}
