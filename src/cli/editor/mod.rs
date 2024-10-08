//! This module provides a collection of input editor functions for various
//! types of user input in Rusty Buddy. It offers functionalities for getting a
//! filename, multiline text, password, and general user input. Each function
//! is imported from its respective module to facilitate user interactions
//! across different contexts, making it easier to gather and manage input
//! from users.
//!
//! ## Overview
//!
//! The input editor aims to enhance user experience by providing intuitive
//! methods for obtaining input, while allowing for features such as
//! auto-completion and password masking. The module consists of the
//! following components:
//!
//! - **Filename Input:** Capture file paths with autocompletion features.
//! - **Multiline Input:** Allow users to provide multiple lines of input.
//! - **Password Input:** Securely gather sensitive data while masking input.
//! - **General User Input:** Capture simple strings from users.
//!
//! ## Example Usages
//!
//! ```rust
//! use crate::cli::editor::get_filename_input;
//!
//! let filename = get_filename_input("Enter filename: ").unwrap();
//! println!("You entered: {}", filename);
//! ```
//!
//! ```rust
//! use crate::cli::editor::get_multiline_input;
//!
//! let user_text = get_multiline_input("Please enter your content: ", vec![]).unwrap();
//! println!("User content:\n{}", user_text);
//! ```
//!
//! ```rust
//! use crate::cli::editor::get_password_input;
//!
//! let password = get_password_input("Enter your password: ").unwrap();
//! println!("Your password is: [hidden]");
//! ```
//!
//! In these examples, you can see how each function is tailored to
//! retrieve user input efficiently, enhancing the overall usability of
//! Rusty Buddy.
mod filename_input_editor;
mod multiline_editor;
mod password_input_editor;
mod user_input_editor;

pub use filename_input_editor::get_filename_input;
pub use multiline_editor::get_multiline_input;
pub use password_input_editor::get_password_input;
pub use user_input_editor::get_user_input;
