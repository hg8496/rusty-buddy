use crate::chat::interface::{ChatStorage, Message};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub struct NilChatStorage {}

impl ChatStorage for NilChatStorage {
    fn load_session(&mut self, _session_name: &str) -> io::Result<Vec<Message>> {
        Ok(Vec::new())
    }

    fn save_session(&self, _session_name: &str, _messages: &Vec<Message>) -> io::Result<()> {
        Ok(())
    }

    fn list_sessions(&self) -> io::Result<Vec<String>> {
        Ok(Vec::new())
    }
}

pub struct DirectoryChatStorage {
    storage_dir: PathBuf,
}

impl DirectoryChatStorage {
    pub fn new(storage_dir: PathBuf) -> Self {
        DirectoryChatStorage { storage_dir }
    }

    fn get_file_path(&self, session_name: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", session_name))
    }

    fn ensure_storage_dir_exists(&self) -> io::Result<()> {
        fs::create_dir_all(&self.storage_dir)?;
        Ok(())
    }
}

impl ChatStorage for DirectoryChatStorage {
    fn load_session(&mut self, session_name: &str) -> io::Result<Vec<Message>> {
        self.ensure_storage_dir_exists()?;
        let file_path = self.get_file_path(session_name);
        let mut file = fs::File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Deserialize the JSON content into a Vec<Message>
        let messages: Vec<Message> = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(messages)
    }

    fn save_session(&self, session_name: &str, messages: &Vec<Message>) -> io::Result<()> {
        self.ensure_storage_dir_exists()?;
        let file_path = self.get_file_path(session_name);
        let mut file = fs::File::create(file_path)?;

        // Serialize the messages to JSON and write to the file
        let json_content = serde_json::to_string(messages)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        file.write_all(json_content.as_bytes())?;
        Ok(())
    }

    fn list_sessions(&self) -> io::Result<Vec<String>> {
        self.ensure_storage_dir_exists()?;
        Ok(fs::read_dir(&self.storage_dir)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.file_name()
                        .to_str()
                        .map(|s| s.trim_end_matches(".json").to_owned())
                })
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::interface::MessageRole;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_storage_dir(nr: u16) -> PathBuf {
        let dir = PathBuf::from(format!(".test_storage{}", nr));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup_test_storage_dir(nr: u16) {
        let dir = PathBuf::from(format!(".test_storage{}", nr));
        if dir.exists() {
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn test_save_and_load_session() {
        let storage_dir = setup_test_storage_dir(1);
        let mut storage = DirectoryChatStorage::new(storage_dir.clone());

        let session_name = "test_session";
        let messages = vec![
            Message {
                role: MessageRole::User,
                content: "Hello, this is a test.".to_string(),
            },
            Message {
                role: MessageRole::Assistant,
                content: "Thank you for the test input.".to_string(),
            },
        ];

        // Save the session
        storage
            .save_session(session_name, &messages)
            .expect("Failed to save session.");

        // Load the session
        let loaded_messages = storage
            .load_session(session_name)
            .expect("Failed to load session.");

        assert_eq!(messages, loaded_messages);

        // Clean up test directories
        cleanup_test_storage_dir(1);
    }

    #[test]
    fn test_list_sessions() {
        let storage_dir = setup_test_storage_dir(2);
        let storage = DirectoryChatStorage::new(storage_dir.clone());

        let session_name_1 = "session_one";
        let session_name_2 = "session_two";

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Dummy content".to_string(),
        }];

        // Save two sessions
        storage
            .save_session(session_name_1, &messages)
            .expect("Failed to save session one.");
        storage
            .save_session(session_name_2, &messages)
            .expect("Failed to save session two.");

        // List sessions
        let session_list = storage.list_sessions().expect("Failed to list sessions.");

        assert!(session_list.contains(&session_name_1.to_string()));
        assert!(session_list.contains(&session_name_2.to_string()));
        assert_eq!(session_list.len(), 2);

        // Clean up
        cleanup_test_storage_dir(2);
    }

    #[test]
    fn test_load_non_existent_session() {
        let storage_dir = setup_test_storage_dir(3);
        let mut storage = DirectoryChatStorage::new(storage_dir.clone());

        let result = storage.load_session("non_existent_session");

        assert!(result.is_err());

        // Clean up
        cleanup_test_storage_dir(3);
    }
}
