use crate::cli::slash_completer::SlashCommandCompleter;
use crate::cli::style::configure_mad_skin;
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

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

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
