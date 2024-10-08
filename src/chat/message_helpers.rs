//! This module provides utility functions for message handling within the chat application.
//!
//! It primarily focuses on retrieving the last assistant message from the chat history,
//! facilitating processes like saving the last response or obtaining context from previous interactions.
//!
//! ## Functions
//!
//! - [`find_last_assistant_message`] - Finds and returns the last message sent by the assistant,
//!   allowing users to interact with and manage AI output efficiently.
//!
use crate::chat::interface::MessageRole;
use crate::chat::service::ChatService;

/// # Function Details
///
/// ## find_last_assistant_message
///
/// Finds the last message from the assistant in the chat history.
///
/// This function iterates through the messages processed by the given `ChatService`,
/// looking for messages where the `role` is `MessageRole::Assistant`. If such a message
/// is found, its content is stored and returned. If no assistant messages are present,
/// the function returns `None`.
///
/// # Arguments
///
/// * `chat_service` - A reference to the `ChatService` that processes the chat messages.
///
/// # Returns
///
/// An `Option<String>`, which contains the content of the last assistant message if it exists,
/// otherwise `None`.
pub fn find_last_assistant_message(chat_service: &ChatService) -> Option<String> {
    let mut last_assistant_message = None;
    chat_service.process_messages(|msg| {
        if msg.role == MessageRole::Assistant {
            last_assistant_message = Some(msg.content.clone());
        }
    });
    last_assistant_message
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::interface::{ChatBackend, ChatStorage, Message, MessageRole};
    use crate::persona::Persona;
    use async_trait::async_trait;
    use std::error::Error;
    use std::io;

    // Mock ChatBackend implementation
    struct MockChatBackend;

    pub struct MockStorageService {
        messages_to_return: Vec<Message>,
    }

    impl MockStorageService {
        pub fn new(messages: Vec<Message>) -> Self {
            MockStorageService {
                messages_to_return: messages,
            }
        }
    }

    impl ChatStorage for MockStorageService {
        fn load_session(&mut self, _session_name: &str) -> io::Result<Vec<Message>> {
            Ok(self.messages_to_return.clone()) // Return a clone of predefined messages
        }

        fn save_session(&self, _session_name: &str, _messages: &[Message]) -> io::Result<()> {
            Ok(())
        }

        fn list_sessions(&self) -> io::Result<Vec<String>> {
            Ok(vec!["session1".to_string(), "session2".to_string()]) // Example session names
        }
    }

    #[async_trait]
    impl ChatBackend for MockChatBackend {
        async fn send_request(
            &mut self,
            _messages: &[Message],
            _use_tools: bool,
        ) -> Result<Message, Box<dyn Error>> {
            Ok(Message::default())
        }

        fn print_statistics(&self) {}
    }

    fn create_mock_persona() -> Persona {
        Persona {
            name: "mock".to_string(),
            chat_prompt: "This is a mock persona for testing purposes.".to_string(),
            file_types: vec!["rs".to_string(), "md".to_string()],
        }
    }

    #[tokio::test]
    async fn test_find_last_assistant_message_with_assistant_messages() {
        let messages = vec![
            Message {
                role: MessageRole::User,
                content: "Hello".to_string(),
                ..Default::default()
            },
            Message {
                role: MessageRole::Assistant,
                content: "Hi, how can I help you?".to_string(),
                ..Default::default()
            },
            Message {
                role: MessageRole::User,
                content: "I'm looking for a Rust library.".to_string(),
                ..Default::default()
            },
            Message {
                role: MessageRole::Assistant,
                content: "I recommend `reqwest` for HTTP requests.".to_string(),
                ..Default::default()
            },
        ];

        let backend = MockChatBackend;
        let storage = MockStorageService::new(messages);
        let mut chat_service = ChatService::new(
            Box::new(backend),
            Box::new(storage),
            create_mock_persona(),
            None,
        );
        chat_service.load_history("history1").unwrap();
        let result = find_last_assistant_message(&chat_service);
        assert_eq!(
            result,
            Some("I recommend `reqwest` for HTTP requests.".to_string())
        );
    }

    #[tokio::test]
    async fn test_find_last_assistant_message_no_assistant_messages() {
        let messages = vec![
            Message {
                role: MessageRole::User,
                content: "Hello".to_string(),
                ..Default::default()
            },
            Message {
                role: MessageRole::User,
                content: "I'm looking for a Rust library.".to_string(),
                ..Default::default()
            },
        ];

        let backend = MockChatBackend;
        let storage = MockStorageService::new(messages);
        let mut chat_service = ChatService::new(
            Box::new(backend),
            Box::new(storage),
            create_mock_persona(),
            None,
        );
        chat_service.load_history("history1").unwrap();

        let result = find_last_assistant_message(&chat_service);
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_find_last_assistant_message_empty_messages() {
        let messages = vec![];

        let backend = MockChatBackend;
        let storage = MockStorageService::new(messages);
        let mut chat_service = ChatService::new(
            Box::new(backend),
            Box::new(storage),
            create_mock_persona(),
            None,
        );
        chat_service.load_history("history1").unwrap();

        let result = find_last_assistant_message(&chat_service);
        assert_eq!(result, None);
    }
}
