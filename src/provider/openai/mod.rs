//! This module provides an interface for interacting with OpenAI's AI backend,
//! facilitating chat interactions and file comparison functionalities.
//!
//! The `OpenAIInterface` struct encapsulates the necessary methods to send
//! messages to the OpenAI API, receive responses, and manage the flow of
//! conversation within the Rusty Buddy application.
//!
//! ## Key Responsibilities
//!
//! - **Message Handling:** Converts custom message formats into the required
//!   formats for the OpenAI API, allowing integration into a chat application.
//! - **Session Management:** Maintains context and state for ongoing conversations,
//!   ensuring seamless interactions with the AI model.
//! - **Backend Integration:** Implements the `ChatBackend` trait for communication
//!   with the OpenAI API, enabling the application to utilize powerful language models.
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::provider::openai::openai_interface::OpenAIInterface;
//!
//! let openai_backend = OpenAIInterface::new("gpt-4".to_string(), 60);
//! // Now openai_backend can be used to send requests and handle responses.
//! ```
//!
//! ## Fields
//!
//! - `model`: A string that specifies the AI model used for generating chat messages.
//! - `timeout_duration`: A duration that represents the timeout for API requests.
//! - `last_call_completion_token`, `last_call_prompt_token`: Track token usage for
//!   the last API call.
//! - `overall_completion_token`, `overall_prompt_token`: Cumulative token usage metrics.
//!
//! ## Methods
//!
//! - **new:** Constructs a new instance of `OpenAIInterface` with specified model and timeout.
//! - **send_request:** Sends a request with messages to the OpenAI backend and retrieves a response.
//! - **print_statistics:** Outputs token usage statistics related to the last request and overall usage.

pub mod file_diff;
pub mod openai_interface;
