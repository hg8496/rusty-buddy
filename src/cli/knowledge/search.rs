use crate::cli::knowledge::knowledge_args::SearchArgs;
use crate::knowledge::{KnowledgeResult, StoreBuilder};
use std::error::Error;

pub async fn search(args: SearchArgs) -> Result<(), Box<dyn Error>> {
    let db = StoreBuilder::new().build().await?;
    // Assuming response.d sata holds embedding data
    let knowledge: Vec<KnowledgeResult> = db.query_knowledge(args.search).await?;
    for piece in knowledge {
        println!("{} {}", piece.data_source, piece.distance);
    }
    Ok(())
}
