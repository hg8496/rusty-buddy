use crate::cli::knowledge::knowledge_args::AddArgs;
use crate::knowledge::{DataSource, EmbeddingData, KnowledgeStore, StoreBuilder};
use log::{info, warn};
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinHandle;
use walkdir::WalkDir;

pub async fn add(add: AddArgs) -> Result<(), Box<dyn Error>> {
    let store = StoreBuilder::new().build().await?;

    if let Some(dir) = add.dir {
        add_directory_to_knowledge(&dir, &store).await?;
    }
    if let Some(file) = add.file {
        add_file_to_knowledge(&file, &store).await?;
    }
    if let Some(url) = add.url {
        add_url_to_knowledge(&url, &store).await?;
    }
    Ok(())
}

async fn add_directory_to_knowledge(
    directory: &PathBuf,
    store: &Arc<dyn KnowledgeStore>,
) -> Result<(), Box<dyn Error>> {
    let tasks: Vec<JoinHandle<()>> = WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| {
            let store = store.clone();
            let path = entry.path().to_owned();
            tokio::spawn(async move { process_file(&path, &store).await })
        })
        .collect();

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn add_file_to_knowledge(
    file: &PathBuf,
    store: &Arc<dyn KnowledgeStore>,
) -> Result<(), Box<dyn Error>> {
    process_file(file, store).await;
    Ok(())
}

async fn process_file(file_path: &PathBuf, store: &Arc<dyn KnowledgeStore>) {
    info!("Processing file: {}", file_path.display());

    // Attempt to read file content
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            warn!("Failed to read file '{}': {}", file_path.display(), e);
            return;
        }
    };

    // Generate an embedding using Cow to avoid unnecessary cloning
    let embedding = match store.get_embedding(Cow::Borrowed(&content)).await {
        Ok(embedding) => embedding,
        Err(e) => {
            warn!(
                "Failed to generate embedding for file '{}': {}",
                file_path.display(),
                e
            );
            return;
        }
    };

    // Store the knowledge without cloning the path or content unnecessarily
    if let Err(e) = store
        .store_knowledge(EmbeddingData {
            data_source: DataSource::LocalFiles(file_path.to_string_lossy().into_owned()),
            embedding: *embedding,
            content: Some(content), // Transfer ownership here
            metadata: None,
        })
        .await
    {
        warn!(
            "Failed to store knowledge for file '{}': {}",
            file_path.display(),
            e
        );
    }
}

async fn add_url_to_knowledge(
    url: &str,
    store: &Arc<dyn KnowledgeStore>,
) -> Result<(), Box<dyn Error>> {
    println!("Processing URL: {}", url);

    // Fetch and process the URL content here using Cow to avoid cloning until necessary
    let content = reqwest::get(url).await?.text().await?;
    let embedding = store.get_embedding(Cow::Borrowed(&content)).await?;

    store
        .store_knowledge(EmbeddingData {
            data_source: DataSource::Internet(url.to_string()),
            embedding: *embedding,
            content: Some(content), // Transfer ownership here
            metadata: None,
        })
        .await?;

    Ok(())
}
