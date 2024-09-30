use crate::chat::context::load_files_into_context;
use crate::chat::interface::ChatStorage;
use crate::chat::interface::{ChatBackend, Message, MessageRole};
use std::error::Error;
use std::path::Path;

// ChatService struct serves as a layer between user input and backend processing.
// It handles storing user messages and interactions, sending messages to the backend,
// and storing/retrieving previous chat sessions.
pub struct ChatService<B: ChatBackend, S: ChatStorage> {
    backend: B,                // Backend responsible for processing the chat messages
    storage: S,                // Storage to persist or load chat sessions
    messages: Vec<Message>,    // In-memory store of current session's messages
    persona: Persona,          // Persona to use
    directory: Option<String>, // Path to context root
}

use crate::persona::Persona;

impl<B: ChatBackend, S: ChatStorage> ChatService<B, S> {
    // Initializes a new ChatService with a given backend and storage.
    pub fn new(backend: B, storage: S, persona: Persona, directory: Option<String>) -> Self {
        ChatService {
            backend,
            storage,
            directory,
            persona,
            messages: vec![],
        }
    }

    pub fn setup_context(&mut self) {
        self.messages.retain(|msg| match msg.role {
            MessageRole::Context => false,
            _ => true,
        });
        if let Some(directory) = &self.directory {
            let mut context = String::from("Use the following Context to assist the user.\n");
            load_files_into_context(Path::new(directory), &self.persona.file_types, &mut context)
                .unwrap();
            self.add_context_message(context.as_str());
        }
        let prompt = self.persona.chat_prompt.clone();
        self.add_context_message(prompt.as_str());
    }

    // Adds a message from the system perspective to the chat messages.
    pub fn add_context_message(&mut self, system_message: &str) {
        self.messages.insert(
            0,
            Message {
                role: MessageRole::Context,
                content: system_message.to_string(),
            },
        )
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

#[cfg(test)]
mod tests {
    use crate::chat::file_storage::NilChatStorage;
    use crate::chat::interface::{ChatBackend, Message};
    use crate::chat::service::ChatService;
    use crate::persona::Persona;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[tokio::test]
    async fn test_setup_context() {
        // Configure a mock persona
        let persona = Persona {
            name: "test".to_string(),
            chat_prompt: "Test persona prompt".to_string(),
            file_types: vec!["rs".to_string()],
        };

        // Construct the path using PathBuf
        let path = PathBuf::from("tests").join("mocks");

        // Create an instance of ChatService
        let mut chat_service = ChatService::new(
            MockChatBackend::new(),
            NilChatStorage {},
            persona.clone(),
            Some(path.to_string_lossy().into_owned()), // Convert to String
        );

        // Set up the context
        chat_service.setup_context();

        // Verify that context messages are correctly set
        assert!(chat_service
            .messages
            .first()
            .unwrap()
            .content
            .contains("Test persona prompt"));

        // Construct the expected mock file path
        let expected_filepath = Path::new("tests").join("mocks").join("mock_file.rs");
        let expected_filename = format!("Filename: {}", expected_filepath.to_string_lossy());

        assert!(chat_service
            .messages
            .last()
            .unwrap()
            .content
            .contains(&expected_filename));
    }

    #[tokio::test]
    async fn test_multiple_setup_context() {
        // Configure a mock persona
        let persona = Persona {
            name: "test".to_string(),
            chat_prompt: "Test persona prompt".to_string(),
            file_types: vec!["rs".to_string()],
        };

        // Construct the path using PathBuf
        let path = PathBuf::from("tests").join("mocks");

        // Create an instance of ChatService
        let mut chat_service = ChatService::new(
            MockChatBackend::new(),
            NilChatStorage {},
            persona.clone(),
            Some(path.to_string_lossy().into_owned()), // Convert to String
        );

        // Set up the context multiple times
        chat_service.setup_context();
        chat_service.setup_context();

        // Verify that context messages are correctly set
        assert!(chat_service
            .messages
            .first()
            .unwrap()
            .content
            .contains("Test persona prompt"));

        // Construct the expected mock file path
        let expected_filepath = Path::new("tests").join("mocks").join("mock_file.rs");
        let expected_filename = format!("Filename: {}", expected_filepath.to_string_lossy());

        assert!(chat_service
            .messages
            .last()
            .unwrap()
            .content
            .contains(&expected_filename));
        assert_eq!(chat_service.messages.len(), 2);
    }

    // Implement a simple ChatBackend mock
    struct MockChatBackend;

    impl MockChatBackend {
        fn new() -> Self {
            MockChatBackend
        }
    }

    impl ChatBackend for MockChatBackend {
        async fn send_request(
            &mut self,
            _messages: &Vec<Message>,
            _use_tools: bool,
        ) -> Result<String, Box<dyn Error>> {
            Ok("".to_string())
        }

        fn print_statistics(&self) {}
    }
}
