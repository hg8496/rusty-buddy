use rustyline::completion::{Completer, Pair};
use rustyline::Context;

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
