use crate::chat::interface::{ChatStorage, Message, MessageRole};
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
        DirectoryChatStorage {
            storage_dir: storage_dir,
        }
    }

    fn get_file_path(&self, session_name: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.txt", session_name))
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

        let messages = content
            .lines()
            .map(|line| Message {
                role: MessageRole::User,
                content: line.to_string(),
            })
            .collect();

        Ok(messages)
    }

    fn save_session(&self, session_name: &str, messages: &Vec<Message>) -> io::Result<()> {
        self.ensure_storage_dir_exists()?;
        let file_path = self.get_file_path(session_name);
        let mut file = fs::File::create(file_path)?;

        for message in messages {
            writeln!(file, "{}", message.content)?;
        }
        Ok(())
    }

    fn list_sessions(&self) -> io::Result<Vec<String>> {
        self.ensure_storage_dir_exists()?;
        Ok(fs::read_dir(&self.storage_dir)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.file_name()
                        .to_str()
                        .map(|s| s.trim_end_matches(".txt").to_owned())
                })
            })
            .collect())
    }
}
