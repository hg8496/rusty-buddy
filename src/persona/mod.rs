pub mod persona;

use crate::config;
pub use persona::Persona;

fn get_internal_persona_configs() -> Vec<Persona> {
    vec![
        Persona {
            name: "rust".to_string(),
            chat_prompt: "You are an experienced Rust developer assisting a colleague with feature development and answering questions related to Rust programming.".to_string()
        },
        Persona {
            name: "swift".to_string(),
            chat_prompt: "You are an experienced Swift developer assisting a colleague with feature development and answering questions related to Swift programming.".to_string()
        },
        Persona {
            name: "java".to_string(),
            chat_prompt: "You are an experienced Java developer assisting a colleague with feature development and answering questions related to Java programming.".to_string()
        },
        Persona {
            name: "typescript".to_string(),
            chat_prompt: "You are an experienced Typescript developer assisting a colleague with feature development and answering questions related to Typescript programming.".to_string()
        },
    ]
}

pub fn get_personas() -> Vec<Persona> {
    let config = config::CONFIG.lock().unwrap();
    [get_internal_persona_configs(), config.personas.clone()].concat()
}

pub fn get_persona(name: &str) -> Option<Persona> {
    get_personas()
        .iter()
        .find(|persona| persona.name == name)
        .cloned()
}
