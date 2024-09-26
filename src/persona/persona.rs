use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Persona {
    pub name: String,
    pub chat_prompt: String,
    pub file_types: Vec<String>,
}
