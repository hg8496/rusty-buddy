use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Context,
}

#[async_trait]
pub trait ChatBackend {
    async fn send_request(
        &mut self,
        messages: &[Message],
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>>;
    fn print_statistics(&self);
}

pub trait ChatStorage {
    fn load_session(&mut self, session_name: &str) -> io::Result<Vec<Message>>;
    fn save_session(&self, session_name: &str, messages: &[Message]) -> io::Result<()>;
    fn list_sessions(&self) -> io::Result<Vec<String>>;
}
