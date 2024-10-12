use crate::cli::knowledge::knowledge_args::InitArgs;
use crate::config;
use crate::config::Config;
use crate::context::{load_files_into_context, ContextConsumer};
use crate::knowledge::DataSource::Context;
use crate::knowledge::{EmbeddingData, KnowledgeStore, StoreBuilder};
use crate::persona::resolve_persona;
use async_channel::{Receiver, Sender};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Bounded channel size as a constant for readability.
const CHANNEL_SIZE: usize = 10;
const MAX_THREADS: usize = 10; // Maximum number of threads

/// Job type alias for clarity
type Job = (String, String);

/// Fetch configuration settings as a clone.
fn get_config() -> Config {
    let config = config::CONFIG.lock().unwrap();
    config.clone()
}

/// Entry point for initializing the knowledge system.
pub async fn init(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let db = StoreBuilder::new().build().await?;
    let (sender, receiver) = async_channel::bounded(CHANNEL_SIZE);
    let persona = resolve_persona(&args.persona, config.default_persona.as_str())?;

    // Spawn workers for processing jobs.
    let handles = spawn_workers(receiver, db);

    // Load and process files
    process_files(sender, &persona)?;

    // Close the channel and wait for workers to finish.
    await_worker_completion(handles).await;

    Ok(())
}

/// Spawn workers that will handle processing jobs concurrently.
fn spawn_workers(receiver: Receiver<Job>, db: Arc<dyn KnowledgeStore>) -> Vec<JoinHandle<()>> {
    (0..MAX_THREADS)
        .map(|_| {
            let receiver_clone = receiver.clone();
            let db_clone = db.clone();

            tokio::spawn(async move {
                process_jobs(receiver_clone, db_clone).await;
            })
        })
        .collect()
}

/// Process jobs received from the channel by retrieving embeddings and saving them to the database.
async fn process_jobs(receiver: Receiver<Job>, db: Arc<dyn KnowledgeStore>) {
    while let Ok((filename, content)) = receiver.recv().await {
        eprintln!("Processing File: {}", filename);

        // Now handle success or failure from embedding processing
        let embedding = match db.get_embedding(content.clone()).await {
            Ok(embedding) => {
                // Assemble the embedding data (once get_embedding has been successful)
                Some(EmbeddingData {
                    data_source: Context(filename.clone()),
                    content: None,
                    embedding: embedding.to_vec(),
                    metadata: None,
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

            if let Err(e) = db.store_knowledge(embedding).await {
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
