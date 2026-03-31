use crate::repl::ReplHelper;
use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

impl Helper for ReplHelper {}
impl Hinter for ReplHelper {
    type Hint = String;
}
impl Highlighter for ReplHelper {}
impl Validator for ReplHelper {}
impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let candidates = vec![
            Pair {
                display: "exit".to_string(),
                replacement: "exit ".to_string(),
            },
            Pair {
                display: "echo".to_string(),
                replacement: "echo ".to_string(),
            },
            Pair {
                display: "cd".to_string(),
                replacement: "cd ".to_string(),
            },
            Pair {
                display: "exec".to_string(),
                replacement: "exec ".to_string(),
            },
            Pair {
                display: "pwd".to_string(),
                replacement: "pwd ".to_string(),
            },
        ];

        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start..pos];

        let matches: Vec<Pair> = candidates
            .into_iter()
            .filter(|c| c.display().starts_with(prefix))
            .collect();

        Ok((start, matches))
    }
}
