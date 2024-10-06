use crate::chat::interface::{ChatStorage, Message};
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct NilChatStorage {}

impl ChatStorage for NilChatStorage {
    fn load_session(&mut self, _session_name: &str) -> io::Result<Vec<Message>> {
        Ok(Vec::new())
    }

    fn save_session(&self, _session_name: &str, _messages: &[Message]) -> io::Result<()> {
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
        let content = fs::read_to_string(file_path)?;

        // Deserialize the JSON content into a Vec<Message>
        let messages: Vec<Message> = serde_json::from_str(&content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        Ok(messages)
    }

    fn save_session(&self, session_name: &str, messages: &[Message]) -> io::Result<()> {
        self.ensure_storage_dir_exists()?;
        let file_path = self.get_file_path(session_name);
        let json_content = serde_json::to_string(messages)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        fs::write(&file_path, json_content.as_bytes())?;
        Ok(())
    }

    fn list_sessions(&self) -> io::Result<Vec<String>> {
        self.ensure_storage_dir_exists()?;
        let mut sessions = fs::read_dir(&self.storage_dir)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                        let metadata = fs::metadata(&path).ok()?;
                        Some((path, metadata.modified().ok()?))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        // Sort by modification time, oldest first
        sessions.sort_by_key(|&(_, modified_time)| modified_time);

        // Extract session names, trimming the `.json` extension
        Ok(sessions
            .iter()
            .filter_map(|(path, _)| {
                path.file_stem()
                    .and_then(|stem| stem.to_str().map(|s| s.to_owned()))
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::interface::MessageRole;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_session() {
        let storage_dir = TempDir::new()
            .expect("Failed to create temp dir")
            .into_path();
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
    }

    #[test]
    fn test_list_sessions() {
        let storage_dir = TempDir::new()
            .expect("Failed to create temp dir")
            .into_path();
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
    }

    #[test]
    fn test_load_non_existent_session() {
        let storage_dir = TempDir::new()
            .expect("Failed to create temp dir")
            .into_path();
        let mut storage = DirectoryChatStorage::new(storage_dir.clone());

        let result = storage.load_session("non_existent_session");

        assert!(result.is_err());
    }
}
