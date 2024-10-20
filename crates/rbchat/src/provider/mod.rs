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
//! ## Important Notes
//!
//! - Ensure that you have the necessary API keys and services configured properly in your
//!   `.env` file for seamless operation.
//! - Both modules are designed to work independently, but can be integrated to enhance
//!   functionality across different AI services, depending on your application's needs.
pub mod ollama;
pub mod openai;
