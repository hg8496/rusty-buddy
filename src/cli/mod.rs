//! This module acts as the main entry point for the various components of the Rusty Buddy application.
//! It organizes different modules related to commands, storage, interfaces, message handling, and service management.
//! Each submodule is responsible for specific functionality within the application, promoting modular design and separation of concerns.
//!
//! ## Key Components
//!
//! - **Chat Module**: Manages chat interactions, allowing users to communicate seamlessly with the AI.
//! - **Commit Message Module**: Facilitates the generation of standardized commit messages based on code changes.
//! - **Create Background Module**: Enables users to create background images using AI, tailored to their specifications.
//! - **Create Icon Module**: Allows the generation of icons based on user input, utilizing AI capabilities to enhance the design process.
//! - **Editor Module**: Provides functionalities for user input, including filename completion, password masking, and multiline editing.
//! - **Initialization Module**: Handles the setup process for Rusty Buddy, including configuration and user input for API keys and model selection.
//! - **Slash Completer Module**: Implements auto-completion for slash commands in the chat interface, enhancing usability.
//! - **Spinner Module**: Displays a visual spinner in the terminal during potentially long-running tasks, improving user experience by indicating processing.
//! - **Style Module**: Configures terminal output styles for a better visual experience while using Rusty Buddy.
//! - **Wish Module**: Implements functionality for users to express wishes for file and directory manipulations, utilizing AI to accomplish user requests.
//!
//! By organizing functionalities into distinct modules, Rusty Buddy makes it easy for users to extend and maintain the application while leveraging the power of AI to improve their development workflows.
pub mod chat;
pub mod commitmessage;
pub mod createbackground;
pub mod createicon;
pub mod editor;
pub mod init;
mod slash_completer;
mod spinner;
mod style;
pub mod wish;
