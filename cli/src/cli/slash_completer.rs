//! This module provides functionality for auto-completing slash commands
//! in a command-line interface (CLI).
//!
//! The `SlashCommandCompleter` struct is responsible for suggesting commands
//! that start with a slash ('/'). It takes a list of available commands and
//! offers completion suggestions based on the current input line.
//!
//! ## Overview
//!
//! The primary purpose of this module is to enhance user experience in CLI
//! applications by providing intelligent autocompletion for commands, allowing
//! users to quickly discover available options. This functionality is crucial
//! in environments where quick command execution is necessary, such as during
//! interactive sessions with an AI assistant.
//!
//! ## Usage
//!
//! Here’s how to use the `SlashCommandCompleter` to enable command completion:
//!
//! ```rust
//! use rustyline::completion::{Completer, Pair};
//! use rustyline::Context;
//! use crate::cli::slash_completer::SlashCommandCompleter;
//!
//! let commands = vec!["help".to_string(), "exit".to_string()];
//! let completer = SlashCommandCompleter::new(commands);
//! ```
//!
//! This snippet initializes a `SlashCommandCompleter` with a list of commands,
//! allowing it to provide suggestions for commands starting with a slash during
//! user input in the CLI.
//!
//! ## Methods
//!
//! The core methods include:
//!
//! - `new`: Creates a new instance of `SlashCommandCompleter` with a given list of commands.
//! - `complete`: Implements the `Completer` trait, providing suggestions for command completion based on user input.
//!
//! ## Example
//!
//! The provided `complete` method can be used within a command-line loop to
//! suggest possible commands as the user types:
//!
//! ```rust
//! let commands = vec!["help".to_string(), "exit".to_string()];
//! let completer = SlashCommandCompleter::new(commands);
//! ```
//!
//! ## Error Handling
//!
//! This module uses the `rustyline` crate for user input management. If there are
//! issues with input processing (e.g., invalid input syntax), appropriate
//! `rustyline::error::ReadlineError` errors will be generated and should be
//! handled gracefully to enhance user experience.
//!
//! Overall, the `SlashCommandCompleter` module empowers CLI applications
//! with impactful UI enhancements, ultimately contributing to more engaging
//! and productive user interactions.

use rustyline::completion::{Completer, Pair};
use rustyline::Context;

/// A completer for slash commands in a command-line interface.
///
/// This struct provides functionality for auto-completing commands that
/// start with a slash ('/'). It takes a list of commands and offers
/// completion suggestions based on the current input line.
///
/// # Examples
///
/// ```
/// let commands = vec!["help".to_string(), "exit".to_string()];
/// let completer = SlashCommandCompleter::new(commands);
/// ```
pub struct SlashCommandCompleter {
    commands: Vec<String>,
}

impl SlashCommandCompleter {
    pub fn new(commands: Vec<String>) -> Self {
        SlashCommandCompleter { commands }
    }
}

impl Completer for SlashCommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        if line.starts_with('/') {
            let mut pairs = vec![];
            for command in &self.commands {
                if command.starts_with(&line[1..pos]) {
                    pairs.push(Pair {
                        display: command.into(),
                        replacement: command.into(),
                    });
                }
            }
            Ok((1, pairs))
        } else {
            Ok((0, vec![]))
        }
    }
}
