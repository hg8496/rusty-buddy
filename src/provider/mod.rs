//! This module serves as an entry point for the `ollama` and `openai` provider modules,
//! allowing them to be imported and used within the broader application context.
//! Each module contains functionalities related to their respective services,
//! enabling easy integration and usage within the application.
//!
//! ## Overview
//!
//! - The **Ollama Module** provides implementations for interacting with the Ollama AI backend,
//!   allowing for sending messages and handling responses.
//! - The **OpenAI Module** facilitates communication with the OpenAI API, enabling various AI functionalities,
//!   including message processing and model management.
//!
//! ## Usage Example
//!
//! To utilize these modules in your Rust application, you can include them as follows:
//!
//! ```rust
//! use crate::provider::{ollama, openai};
//!
//! // Example usage of Ollama integration
//! let ollama_instance = ollama::ollama_interface::OllamaInterface::new("llama2".to_string(), None);
//!
//! // Example usage of OpenAI integration
//! let openai_instance = openai::openai_interface::OpenAIInterface::default();
//! ```
//!
//! ## Important Notes
//!
//! - Ensure that you have the necessary API keys and services configured properly in your
//!   `.env` file for seamless operation.
//! - Both modules are designed to work independently, but can be integrated to enhance
//!   functionality across different AI services, depending on your application's needs.
pub mod ollama;
pub mod openai;
