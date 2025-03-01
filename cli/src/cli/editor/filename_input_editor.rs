//! This module provides a command-line interface helper that allows for
//! filename autocompletion and highlighting effects. It utilizes the `rustyline`
//! crate to handle user input, completion, and visual styles. The `get_filename_input`
//! function prompts the user for a filename input, incorporating enhanced features
//! such as syntax highlighting, command history navigation, and inline completion.
//!
//! The `MyHelper` struct implements the `Helper`, `Completer`, `Hinter`,
//! and `Validator` traits to integrate filename completion and highlighting within
//! the readline interface. The completion suggestions are based on the filesystem
//! and enhanced by custom styling.
//!
//! # Functions
//!
//! - `get_filename_input(prompt: &str)`: Displays a prompt to the user and
//!   retrieves a valid filename input. It handles potential errors gracefully,
//!   returning an empty string upon interruption or EOF.
//!
//! ## Example
//!
//! ```rust
//! let filename = get_filename_input("Enter filename: ").unwrap();
//! println!("You entered: {}", filename);
//! ```
use std::borrow::Cow::{self, Borrowed, Owned};

use crate::cli::style::configure_mad_skin;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent, Validator};
use rustyline::{Completer, Helper, Hinter};

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

/// Prompts the user for a filename input, displaying a styled prompt and
/// providing filename autocompletion features. Returns the entered filename
/// as a trimmed string. This function handles potential errors gracefully,
/// returning an empty string on interruption or EOF.
///
/// # Arguments
///
/// * `prompt` - A string slice that will be displayed as a prompt for the user.
///
/// # Returns
///
/// This function returns a `Result<String, Box<dyn std::error::Error>>`, where:
/// - `Ok(String)` contains the user input trimmed of whitespace,
/// - `Ok(String::new())` is returned if input is interrupted or EOF is received,
/// - `Err(Box<dyn std::error::Error>)` contains an error if reading input fails.
///
/// # Example
/// ```rust
/// let input = get_filename_input("Enter your filename: ")?;
/// println!("You entered: {}", input);
/// ```
pub fn get_filename_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

    // Print a styled prompt
    let skin = configure_mad_skin();
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored
    match rl.readline("") {
        Ok(input) => Ok(input.trim().to_string()),
        Err(ReadlineError::Interrupted) => Ok(String::new()), // Return empty string as cancellation
        Err(ReadlineError::Eof) => Ok(String::new()),
        Err(err) => Err(Box::new(err)),
    }
}
