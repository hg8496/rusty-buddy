//! This module defines the `ChatService` struct, which acts as a mediator
//! between user interactions and backend processing. It is responsible for
//! managing session messages, interfacing with storage, and communicating
//! user messages to a backend service. The `ChatService` encapsulates the
//! entire chat ecosystem, handling context setup, message processing, and
//! state management for effective AI interaction.
//!
//! ## Key Responsibilities
//!
//! - **Session Management:** Track user interactions, assistant responses,
//!   and context-related messages throughout the chat session.
//! - **Storage Interface:** Load and save session data, managing persistence
//!   through the designated storage strategy (such as `ChatStorage`).
//! - **Backend Communication:** Send user messages to the configured AI backend
//!   (such as OpenAI or Ollama) and retrieve responses for further processing.
//! - **Context Setup:** Establish context by loading relevant files and
//!   integrating them into the chat session for responsive interactions.
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::chat::service::ChatService;
//! use crate::chat::interface::{ChatBackend, ChatStorage};
//! use crate::persona::Persona;
//!
//! // Create an instance of a backend and storage
//! let backend: Box<dyn ChatBackend> = ...; // Your implementation
//! let storage: Box<dyn ChatStorage> = ...; // Your implementation
//! let persona = Persona { /* initialization data */ };
//!
//! // Initialize the ChatService
//! let mut chat_service = ChatService::new(backend, storage, persona, None);
//!
//! // Start the chat session
//! chat_service.setup_context();
//! ```
//!
//! ## Methods
//!
//! ### `new`
//!
//! Constructs a new `ChatService`.
//!
//! ### `setup_context`
//!
//! Sets up the initial context for the chat session, including loading files.
//!
//! ### `send_message`
//!
//! Sends a user message to the backend, retrieves the assistant's response,
//! and records it in the session.
//!
//! ### `load_history`
//!
//! Loads previous chat messages from storage by session name.
//!
//! ### `save_history`
//!
//! Saves current chat messages to storage under a given session name.
//!
//! ### `print_statistics`
//!
//! Outputs statistics detailing the usage of the chat session.

// The `ChatService` struct encapsulates the entirety of chat session management.
use crate::chat::context::load_files_into_context;
use crate::chat::interface::ChatStorage;
use crate::chat::interface::{ChatBackend, Message, MessageRole};
use crate::chat::service_builder::ChatServiceBuilder;
use std::error::Error;
use std::path::{Path, PathBuf};

/// The `ChatService` struct acts as a mediator between user interactions and backend processing.
/// It is responsible for managing session messages, interfacing with storage, and communicating user messages to a backend service. It handles context setup, message processing, and state management between the user and the chat backend.
pub struct ChatService {
    backend: Box<dyn ChatBackend>, // Handles message processing and interactions with OpenAI or others
    storage: Box<dyn ChatStorage>, // Manages storing and loading previous chat sessions
    messages: Vec<Message>,        // Stores messages exchanged during the current chat session
    persona: Persona,              // Represents the context and behavior in the chat session
    directory: Option<Vec<PathBuf>>, // Directory path for file context loading
}

use crate::persona::Persona;

// Implementation of ChatService struct
impl ChatService {
    // Constructor to initialize a new ChatService with a backend and storage system.
    pub fn new(
        backend: Box<dyn ChatBackend>,
        storage: Box<dyn ChatStorage>,
        persona: Persona,
        directory: Option<Vec<PathBuf>>,
    ) -> Self {
        let mut cs = ChatService {
            backend,
            storage,
            directory,
            persona,
            messages: vec![],
        };
        // Add persona's chat prompt to the context
        let prompt = cs.persona.chat_prompt.clone();
        cs.add_system_message(prompt.as_str());

        cs
    }

    pub fn builder() -> ChatServiceBuilder {
        ChatServiceBuilder::default()
    }
    // Sets up the initial context for the chat session, including loading files.
    pub fn setup_context(&mut self) {
        // Remove existing context messages
        self.messages
            .retain(|msg| !matches!(msg.role, MessageRole::Context));
        // Load files into context from a specified directory
        self.add_context_message(
            "The following context are the information I need, to assist the user.\nContext:\n",
        );
        if let Some(directories) = self.directory.clone() {
            for directory in directories {
                load_files_into_context(
                    self,
                    Path::new(&directory),
                    self.persona.file_types.clone().as_slice(),
                )
                .unwrap();
            }
        }
    }

    // Inserts a new context message into the session
    pub fn add_context_message(&mut self, system_message: &str) {
        let mut pos = 1; // 0 is the persona prompt
        for (i, m) in self.messages.iter().enumerate() {
            if m.role == MessageRole::Context {
                pos = i;
            }
        }
        self.messages.insert(
            pos,
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
    use async_trait::async_trait;
    use std::env;
    use std::error::Error;

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
        let path = env::current_dir()
            .unwrap()
            .join("tests")
            .join("mock_dirs")
            .join("dir1")
            .canonicalize()
            .unwrap();
        // Create an instance of ChatService
        let mut chat_service = ChatService::new(
            Box::new(MockChatBackend::new()),
            Box::new(NilChatStorage {}),
            persona.clone(),
            Some(vec![path]), // Convert to String
        );

        // Set up the context
        chat_service.setup_context();

        // Verify that context messages are correctly set
        assert_eq!(
            "Test persona prompt",
            chat_service.messages.first().unwrap().content
        );

        assert!(chat_service
            .messages
            .iter()
            .any(|message| message.content.contains("mock_file.rs")));
        assert_eq!(chat_service.messages.len(), 4);
    }

    #[tokio::test]
    async fn test_setup_context_multi_dir() {
        // Define a mock persona for testing
        let persona = Persona {
            name: "test".to_string(),
            chat_prompt: "Test persona prompt".to_string(),
            file_types: vec!["rs".to_string()],
        };

        // Construct the path using PathBuf
        let base_path = env::current_dir().unwrap().join("tests").join("mock_dirs");
        let path1 = base_path.join("dir1").canonicalize().unwrap();
        let path2 = base_path.join("dir2").canonicalize().unwrap();

        // Create an instance of ChatService
        let mut chat_service = ChatService::new(
            Box::new(MockChatBackend::new()),
            Box::new(NilChatStorage {}),
            persona.clone(),
            Some(vec![path1, path2]), // Convert to String
        );

        // Set up the context
        chat_service.setup_context();

        // Verify that context messages are correctly set
        assert_eq!(
            "Test persona prompt",
            chat_service.messages.first().unwrap().content
        );
        eprintln!("{:?}", chat_service.messages);
        assert!(chat_service
            .messages
            .iter()
            .any(|message| message.content.contains("mock_file.rs")));

        assert!(chat_service
            .messages
            .iter()
            .any(|message| message.content.contains("mock_file2.rs")));

        assert!(chat_service
            .messages
            .iter()
            .any(|message| message.content.contains("mock_file3.rs")));
        assert_eq!(chat_service.messages.len(), 5);
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
        let path = env::current_dir()
            .unwrap()
            .join("tests")
            .join("mock_dirs")
            .join("dir1")
            .canonicalize()
            .unwrap();

        // Create an instance of ChatService
        let mut chat_service = ChatService::new(
            Box::new(MockChatBackend::new()),
            Box::new(NilChatStorage {}),
            persona.clone(),
            Some(vec![path]), // Convert to String
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

        assert!(chat_service
            .messages
            .iter()
            .any(|message| message.content.contains("mock_file.rs")));
        assert_eq!(chat_service.messages.len(), 4);
    }

    // Implement a simple ChatBackend mock
    struct MockChatBackend;

    impl MockChatBackend {
        fn new() -> Self {
            MockChatBackend
        }
    }

    #[async_trait]
    impl ChatBackend for MockChatBackend {
        async fn send_request(
            &mut self,
            _messages: &[Message],
            _use_tools: bool,
        ) -> Result<String, Box<dyn Error>> {
            Ok("".to_string())
        }

        fn print_statistics(&self) {}
    }
}
