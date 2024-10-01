use crate::chat::context::load_files_into_context;
use crate::chat::interface::ChatStorage;
use crate::chat::interface::{ChatBackend, Message, MessageRole};
use std::error::Error;
use std::path::Path;

// The ChatService struct acts as a mediator between user interactions and backend processing.
// It is responsible for managing session messages, interfacing with storage,
// and communicating user messages to a backend service.
pub struct ChatService<B: ChatBackend, S: ChatStorage> {
    backend: B,                // Handles message processing and interactions with OpenAI or others
    storage: S,                // Manages storing and loading previous chat sessions
    messages: Vec<Message>,    // Stores messages exchanged during the current chat session
    persona: Persona,          // Represents the context and behavior in the chat session
    directory: Option<String>, // Directory path for file context loading
}

use crate::persona::Persona;

// Implementation of ChatService struct
impl<B: ChatBackend, S: ChatStorage> ChatService<B, S> {
    // Constructor to initialize a new ChatService with a backend and storage system.
    pub fn new(backend: B, storage: S, persona: Persona, directory: Option<String>) -> Self {
        ChatService {
            backend,
            storage,
            directory,
            persona,
            messages: vec![],
        }
    }

    // Sets up the initial context for the chat session, including loading files.
    pub fn setup_context(&mut self) {
        // Remove existing context messages
        self.messages.retain(|msg| match msg.role {
            MessageRole::Context => false,
            _ => true,
        });
        // Load files into context from a specified directory
        if let Some(directory) = &self.directory {
            let mut context = String::from("Use the following context to assist the user.\n");
            load_files_into_context(Path::new(directory), &self.persona.file_types, &mut context)
                .unwrap();
            self.add_context_message(context.as_str());
        }
        // Add persona's chat prompt to the context
        let prompt = self.persona.chat_prompt.clone();
        self.add_context_message(prompt.as_str());
    }

    // Inserts a new context message into the session
    pub fn add_context_message(&mut self, system_message: &str) {
        self.messages.insert(
            0,
            Message {
                role: MessageRole::Context,
                content: system_message.to_string(),
            },
        );
    }

    // Adds a system message to the session's messages
    pub fn add_system_message(&mut self, system_message: &str) {
        self.messages.push(Message {
            role: MessageRole::System,
            content: system_message.to_string(),
        });
    }

    // Helper method to apply a function to all messages being processed
    pub fn process_messages<F>(&self, mut func: F)
    where
        F: FnMut(&Message),
    {
        for msg in self.messages.iter() {
            func(msg);
        }
    }

    // Sends a user message to the backend, potentially using tools, and captures the response
    pub async fn send_message(
        &mut self,
        user_message: &str,
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>> {
        // Add the user message to the session messages
        self.messages.push(Message {
            role: MessageRole::User,
            content: user_message.to_string(),
        });
        // Send the request to the backend service and capture the response
        let response = self.backend.send_request(&self.messages, use_tools).await?;
        // Store the assistant's response message
        self.messages.push(Message {
            role: MessageRole::Assistant,
            content: response.clone(),
        });
        Ok(response)
    }

    // Loads chat history from storage by session name
    pub fn load_history(&mut self, session_name: &str) -> Result<(), Box<dyn Error>> {
        self.messages = self.storage.load_session(session_name)?;
        Ok(())
    }

    // Saves current chat messages to storage with a specified session name
    pub fn save_history(&self, session_name: &str) -> Result<(), Box<dyn Error>> {
        self.storage.save_session(session_name, &self.messages)?;
        Ok(())
    }

    // Outputs chat session statistics using the backend's built-in function
    pub fn print_statistics(&self) {
        self.backend.print_statistics();
    }
}

// Unit tests for ChatService
#[cfg(test)]
mod tests {
    use crate::chat::file_storage::NilChatStorage;
    use crate::chat::interface::{ChatBackend, Message};
    use crate::chat::service::ChatService;
    use crate::persona::Persona;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    // Test function for the setup_context method
    #[tokio::test]
    async fn test_setup_context() {
        // Define a mock persona for testing
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

    // Test function for multiple invocations of setup_context
    #[tokio::test]
    async fn test_multiple_setup_context() {
        // Define a mock persona for testing
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