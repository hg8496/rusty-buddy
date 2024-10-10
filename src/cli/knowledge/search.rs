use crate::cli::knowledge::knowledge_args::SearchArgs;
use crate::config;
use crate::config::{get_knowledge_dir, Config};
use crate::knowledge::EmbeddingServiceBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Search {
    distance: f32,
    file_name: String,
}

fn get_config() -> Config {
    let config = config::CONFIG.lock().unwrap();
    config.clone()
}

pub async fn search(args: SearchArgs) -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let db = Surreal::new::<RocksDb>(get_knowledge_dir()?.to_str().unwrap()).await?;
    db.use_ns("knowledge").use_db("knowledge_db").await?;
    let model_name = config.ai.embedding_model.clone();
    let client = EmbeddingServiceBuilder::new()
        .model_name(model_name.into())
        .build()?;
    let embedding = client.inner.get_embedding(args.search).await?;
    db.query("DEFINE INDEX hnsw_pts ON embedding_table FIELDS embedding HNSW DIMENSION 3072;")
        .await?;
    // Assuming response.data holds embedding data
    let mut groups = db
        .query("SELECT file_name, vector::similarity::cosine(embedding, $embedding) AS distance FROM embedding_table WHERE embedding <|10,40|> $embedding ORDER BY distance;")
        .bind(("embedding", embedding))
        .await?;
    let files: Vec<Search> = groups.take(0)?;
    for file in files {
        println!("{} {}", file.file_name, file.distance);
    }
    Ok(())
}
