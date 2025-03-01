//! This module provides functionality to securely read password input from the user,
//! masking the input characters with asterisks, and ensuring that the password is not
//! saved in the command history. It utilizes the `rustyline` library for line editing and
//! handling user input in a terminal setting, providing an intuitive and secure way for
//! users to enter sensitive information.
//!
//! # Key Features
//!
//! - **Input Masking:** The input is obscured with asterisks as users type, ensuring privacy.
//! - **Security:** Passwords entered are not saved in the command history, preventing unauthorized access.
//!
//! # Functions
//!
//! ## `get_password_input`
//!
//! Prompts the user for password input and returns the entered password as a `String`.
//! The input is masked to enhance security.
//!
//! ### Arguments
//!
//! - `prompt`: A string slice to display as a prompt for user input.
//!
//! ### Returns
//!
//! - `Result<String>`: Contains the user input if successful or an error if reading input fails.
//!
//! ### Example
//!
//! ```rust
//! use rustyline::error::ReadlineError;
//! use crate::cli::editor::get_password_input;
//!
//! match get_password_input("Enter your password: ") {
//!     Ok(password) => println!("Your password is: [hidden]"),
//!     Err(e) => eprintln!("Error reading password: {}", e),
//! }
//! ```
//!
//! This example demonstrates a simple use case where the user is prompted for their password,
//! and the input is masked to maintain confidentiality.

use rustyline::config::Configurer;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::{ColorMode, Editor, Result};
use rustyline::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

#[derive(Completer, Helper, Hinter, Validator)]
struct MaskingHighlighter {
    masking: bool,
}

impl Highlighter for MaskingHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        use unicode_width::UnicodeWidthStr;
        if self.masking {
            Owned("*".repeat(line.width()))
        } else {
            Borrowed(line)
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool {
        self.masking
    }
}

pub fn get_password_input(prompt: &str) -> Result<String> {
    let h = MaskingHighlighter { masking: false };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(h));

    rl.helper_mut().expect("No helper").masking = true;
    rl.set_color_mode(ColorMode::Forced); // force masking
    rl.set_auto_add_history(false); // prevent storing passwords in history
    let mut guard = rl.set_cursor_visibility(false)?;

    let password = rl.readline(prompt)?;
    guard.take();

    Ok(password)
}
