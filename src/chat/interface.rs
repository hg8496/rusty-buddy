use std::error::Error;
use std::io;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

pub trait ChatBackend {
    async fn send_request(
        &mut self,
        messages: &Vec<Message>,
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>>;
    fn print_statistics(&self);
}

pub trait ChatStorage {
    fn load_session(&mut self, session_name: &str) -> io::Result<Vec<Message>>;
    fn save_session(&self, session_name: &str, messages: &Vec<Message>) -> io::Result<()>;
    fn list_sessions(&self) -> io::Result<Vec<String>>;
}
