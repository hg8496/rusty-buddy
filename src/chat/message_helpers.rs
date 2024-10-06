use crate::chat::interface::MessageRole;
use crate::chat::service::ChatService;

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
        ) -> Result<String, Box<dyn Error>> {
            Ok(String::new())
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
            },
            Message {
                role: MessageRole::Assistant,
                content: "Hi, how can I help you?".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "I'm looking for a Rust library.".to_string(),
            },
            Message {
                role: MessageRole::Assistant,
                content: "I recommend `reqwest` for HTTP requests.".to_string(),
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
            },
            Message {
                role: MessageRole::User,
                content: "I'm looking for a Rust library.".to_string(),
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
