use crate::cli::slash_completer::SlashCommandCompleter;
use crate::cli::style::configure_mad_skin;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::MatchingBracketValidator;
use rustyline::EditMode::Emacs;
use rustyline::{Completer, Config, DefaultEditor, Editor, Helper, Hinter, Validator};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::error::Error;

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: SlashCommandCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
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

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

// Function to capture user input using rustyline with multiline support.
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
        validator: MatchingBracketValidator::new(),
    };
    rl.set_helper(Some(h));
    let mut buffer = String::new();

    // Create a MadSkin for styling the prompt
    let skin = configure_mad_skin();
    // Use termimad to print a horizontal line and a colored prompt
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    loop {
        match rl.readline("") {
            Ok(line) => {
                if line.trim_start().starts_with('/') {
                    // Exit the loop if a slash command is entered
                    buffer.push_str(&line); // Keep the command in the buffer if needed
                    break;
                }
                buffer.push_str(&line);
                buffer.push('\n');
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(buffer)
}
pub fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::builder().edit_mode(Emacs).build();
    let mut rl = DefaultEditor::with_config(config)?;

    // Print a styled prompt
    let skin = configure_mad_skin();
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    // Read a single line of input
    match rl.readline("") {
        Ok(input) => {
            // Valid input or cancel action if empty
            Ok(input.trim().to_string())
        }
        Err(ReadlineError::Interrupted) => {
            Ok(String::new()) // Return empty, indicating cancel/use default
        }
        Err(ReadlineError::Eof) => {
            Ok(String::new()) // Return empty for end-of-file signal
        }
        Err(err) => Err(Box::new(err)),
    }
}
