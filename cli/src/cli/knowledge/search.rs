use crate::cli::knowledge::knowledge_args::SearchArgs;
use rbchat::knowledge::{KnowledgeResult, StoreBuilder};
use std::error::Error;

pub async fn search(args: SearchArgs) -> Result<(), Box<dyn Error>> {
    let db = StoreBuilder::new().build().await?;
    let knowledge: Vec<KnowledgeResult> = db
        .query_knowledge(args.search.into(), args.limit.unwrap_or(10))
        .await?;
    for piece in knowledge {
        println!("{} {}", piece.data_source, piece.distance);
    }
    Ok(())
}
