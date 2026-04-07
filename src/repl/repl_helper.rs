use crate::commands::validator::get_executable_commands;
use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct ReplHelper {
    external_executables: Vec<Pair>,
}

impl ReplHelper {
    pub(crate) fn new() -> Self {
        ReplHelper {
            external_executables: build_executables_list(),
        }
    }
}

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
        let build_ins = vec![
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

        let candidates: Vec<Pair> = build_ins
            .into_iter()
            .chain(self.external_executables.clone().into_iter())
            .collect();

        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start..pos];

        let matches: Vec<Pair> = candidates
            .into_iter()
            .filter(|c| c.display().starts_with(prefix))
            .collect();

        Ok((start, matches))
    }
}

fn build_executables_list() -> Vec<Pair> {
    let executables: Vec<String> = match std::env::var("PATH") {
        Ok(path) => get_executable_commands(path),
        Err(_) => return Vec::new(),
    };
    let mut executables_pair_list: Vec<Pair> = Vec::new();

    for cmd_with_full_path in executables {
        let executable = cmd_with_full_path.split("/").last().unwrap();

        executables_pair_list.push(Pair {
            display: executable.to_string(),
            replacement: format!("{} ", executable).to_string(),
        })
    }

    executables_pair_list.sort_by(|a, b| a.display.cmp(&b.display));

    executables_pair_list
}
