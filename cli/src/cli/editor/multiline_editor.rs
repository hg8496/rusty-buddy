// This module facilitates the collection of multiline user input with command
// completion and syntax highlighting using the `rustyline` library. It defines
// a custom `MyHelper` struct that implements various traits necessary for
// providing completion and highlighting functionality. The main function
// `get_multiline_input` displays a styled prompt, reads user input line by
// line, and returns the complete input as a single string. The user can enter
// commands starting with a '/', and the prompt can be configured for different
// aesthetics.
//
// ## Key Components
//
// - **MyHelper**: Implements the `Helper`, `Completer`, `Hinter`, and `Validator`
//   traits from `rustyline` to provide features such as inline completion and
//   syntax highlighting for user inputs.
//
// - **get_multiline_input**: The primary function of this module, responsible
//   for prompting the user for multiline input, handling completion requests, and
//   storing the result for further processing.
//
// ## Example Usage
//
// ```rust
// use crate::cli::editor::get_multiline_input;
//
// let user_input = get_multiline_input("Please enter your input: ", vec![]).unwrap();
// println!("User input: {}", user_input);
// ```
//
// In the example above, the `get_multiline_input` function is called, allowing
// the user to enter multiple lines of text. It returns the complete input as a
// single string, which can then be processed or displayed.
//
// ## Important Notes
//
// - The module uses a custom completion strategy via the `SlashCommandCompleter`
//   for commands prefixed with a slash ('/').
// - Syntax highlighting is facilitated by the `MatchingBracketHighlighter`, which
//   enhances input readability and user experience.
use crate::cli::slash_completer::SlashCommandCompleter;
use crate::cli::style::configure_mad_skin;
use rustyline::highlight::CmdKind;
use rustyline::{
    error::ReadlineError,
    highlight::{Highlighter, MatchingBracketHighlighter},
    hint::HistoryHinter,
    history::DefaultHistory,
    Completer, Editor, Helper, Hinter, Validator,
};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::error::Error;

/// This struct implements various traits to integrate filename completion
/// and syntax highlighting into the `rustyline` editor.
#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: SlashCommandCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
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

/// Prompts the user for multiline input with command completions and returns
/// the complete input as a single string. The input will be split into lines
/// based on user input until an end signal (like Ctrl+D) is received.
///
/// # Arguments
///
/// * `prompt` - A string slice to display as a prompt for user input.
/// * `slash_completions` - A vector of command completions to provide in the
///   editor.
///
/// # Returns
///
/// This function returns a `Result<String, Box<dyn std::error::Error>>`
/// containing:
/// - `Ok(String)` with the combined input from the user,
/// - `Ok(String::new())` if input is interrupted or EOF occurs,
/// - `Err(Box<dyn std::error::Error>)` if reading input fails.
pub fn get_multiline_input(
    prompt: &str,
    slash_completions: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let mut rl: Editor<MyHelper, DefaultHistory> = Editor::new()?;
    let completer = SlashCommandCompleter::new(slash_completions);
    let h = MyHelper {
        completer,
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: "".to_owned(),
    };
    rl.set_helper(Some(h));

    let mut buffer: String = String::new();
    let skin = configure_mad_skin(); // Configure for styling the prompt
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    loop {
        match rl.readline("") {
            Ok(line) => {
                if line.trim_start().starts_with('/') {
                    buffer.push_str(&line); // Keep the command in the buffer if needed
                    break;
                }
                buffer.push_str(&line);
                buffer.push('\n');
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(buffer)
}
