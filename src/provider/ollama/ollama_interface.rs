use crate::chat::interface::{ChatBackend, Message, MessageRole};
use async_trait::async_trait;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse},
    IntoUrlSealed, Ollama,
};
use std::error::Error;

pub struct OllamaInterface {
    ollama: Ollama,
    model: String,
}

impl OllamaInterface {
    pub fn new(model: String, ourl: Option<String>) -> Self {
        let url = ourl.unwrap_or("http://localhost:11434".into());
        OllamaInterface {
            ollama: Ollama::from_url(url.clone().into_url().unwrap()),
            model,
        }
    }

    fn convert_messages(messages: &[Message]) -> Vec<ChatMessage> {
        let mut chat_messages: Vec<ChatMessage> = Vec::new();

        // Convert Message into ChatMessage for ollama
        for msg in messages {
            match msg.role {
                MessageRole::User => {
                    chat_messages.push(ChatMessage::user(msg.content.clone()));
                }
                MessageRole::Assistant => {
                    chat_messages.push(ChatMessage::assistant(msg.content.clone()));
                }
                MessageRole::Context => {
                    chat_messages.push(ChatMessage::system(msg.content.clone()));
                }
                MessageRole::System => {
                    chat_messages.push(ChatMessage::system(msg.content.clone()));
                }
            }
        }
        chat_messages
    }
}

#[async_trait]
impl ChatBackend for OllamaInterface {
    async fn send_request(
        &mut self,
        messages: &[Message],
        _use_tools: bool,
    ) -> Result<String, Box<dyn Error>> {
        let chat_messages = Self::convert_messages(messages);

        let request = ChatMessageRequest::new(self.model.clone(), chat_messages.clone());
        let responseo: ChatMessageResponse = self.ollama.send_chat_messages(request).await?;

        let mut content = String::new();

        if let Some(assistant_message) = responseo.message {
            content += &assistant_message.content;
        }
        Ok(content)
    }

    fn print_statistics(&self) {
        // Implement statistics if required
        println!("Using Ollama model: {}", self.model);
    }
}
