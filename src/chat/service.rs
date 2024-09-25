use crate::chat::interface::ChatStorage;
use crate::chat::interface::{ChatBackend, Message, MessageRole};
use std::error::Error;

// ChatService struct serves as a layer between user input and backend processing.
// It handles storing user messages and interactions, sending messages to the backend,
// and storing/retrieving previous chat sessions.
pub struct ChatService<B: ChatBackend, S: ChatStorage> {
    backend: B,             // Backend responsible for processing the chat messages
    storage: S,             // Storage to persist or load chat sessions
    messages: Vec<Message>, // In-memory store of current session's messages
}

impl<B: ChatBackend, S: ChatStorage> ChatService<B, S> {
    // Initializes a new ChatService with a given backend and storage.
    pub fn new(backend: B, storage: S) -> Self {
        ChatService {
            backend,
            storage,
            messages: vec![],
        }
    }

    // Adds a message from the system perspective to the chat messages.
    pub fn add_system_message(&mut self, system_message: &str) {
        self.messages.push(Message {
            role: MessageRole::System,
            content: system_message.to_string(),
        })
    }

    // Sends a user's message to the backend, potentially using any configured tools,
    // and saves the response as an assistant's message.
    pub async fn send_message(
        &mut self,
        user_message: &str,
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>> {
        self.messages.push(Message {
            role: MessageRole::User,
            content: user_message.to_string(),
        });
        let response = self.backend.send_request(&self.messages, use_tools).await?;
        self.messages.push(Message {
            role: MessageRole::Assistant,
            content: response.clone(),
        });
        Ok(response)
    }

    // Loads chat history from storage into the current message set, using the provided session name.
    pub fn load_history(&mut self, session_name: &str) -> Result<(), Box<dyn Error>> {
        self.messages = self.storage.load_session(session_name)?;
        Ok(())
    }

    // Stores the current chat messages into storage with a given session name.
    pub fn save_history(&self, session_name: &str) -> Result<(), Box<dyn Error>> {
        self.storage.save_session(session_name, &self.messages)?;
        Ok(())
    }

    // Prints statistics related to the chat session, using the backend's statistics function.
    pub fn print_statistics(&self) {
        self.backend.print_statistics();
    }
}
