use crate::cli::knowledge::knowledge_args::InitArgs;
use crate::config;
use crate::config::{get_knowledge_dir, Config};
use crate::context::{load_files_into_context, ContextConsumer};
use crate::knowledge::EmbeddingServiceBuilder;
use crate::persona::resolve_persona;
use async_channel::Receiver;
use async_channel::Sender;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

/// Bounded channel size                                                                     
const CHANNEL_SIZE: usize = 10;

type Job = (String, String);

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub file_name: String,
    pub embedding: Vec<f32>, // Adjust the size based on the model
    pub metadata: Option<String>,
}

// Add this to your SurrealDB record structure
#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

fn get_config() -> Config {
    let config = config::CONFIG.lock().unwrap();
    config.clone()
}

pub async fn init(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let db = Surreal::new::<RocksDb>(get_knowledge_dir()?.to_str().unwrap()).await?;
    db.use_ns("knowledge").use_db("knowledge_db").await?;
    let (sender, receiver) = async_channel::bounded(CHANNEL_SIZE);
    let persona = resolve_persona(&args.persona, config.default_persona.as_str())?;
    let model_name = if let Some(model) = args.model {
        model // Use user-specified model
    } else {
        config.ai.embedding_model.clone() // Default model from configuration
    };
    let client = EmbeddingServiceBuilder::new()
        .model_name(model_name.into())
        .build()?;

    let max_threads = 10; // Define the maximum number of threads
    let mut handles = vec![];

    // Spawn a fixed number of threads
    for _ in 0..max_threads {
        let client_clone = client.clone();
        let db_clone = db.clone();
        let receiver_clone: Receiver<Job> = receiver.clone();

        let handle = tokio::spawn(async move {
            loop {
                let job = receiver_clone.recv().await;
                let (filename, content) = match job {
                    Ok(job) => job,
                    Err(_) => break,
                };
                eprintln!("Processing File: {}", filename);

                let embedding_response = client_clone.inner.get_embedding(content).await.unwrap();

                let embedding_data = EmbeddingData {
                    file_name: filename.clone(),
                    embedding: embedding_response.to_vec(),
                    metadata: Some("Add additional details here if needed".to_string()),
                };

                let _: Option<Record> = db_clone
                    .create("embedding_table")
                    .content(embedding_data)
                    .await
                    .unwrap();
            }
        });
        handles.push(handle);
    }

    // Initialize the OpenAI client
    let mut consumer = Consumer { sender };
    load_files_into_context(
        &mut consumer,
        &env::current_dir().unwrap(),
        &persona.file_types,
    )
    .unwrap();

    // Close the channel after loading files
    drop(consumer);

    // Wait for all threads to finish processing
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

struct Consumer {
    sender: Sender<Job>,
}

impl ContextConsumer for Consumer {
    fn consume(&mut self, filename: Cow<str>, content: Cow<str>) -> Result<(), Box<dyn Error>> {
        let job = (filename.into(), content.into());
        // Use block_in_place to execute blocking operations
        tokio::task::block_in_place(|| self.sender.send_blocking(job))?;
        Ok(())
    }
}
