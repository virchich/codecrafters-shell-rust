use crate::commands::validator::get_executable_commands;
use crate::repl::complete_store;
use crate::supported_envs::SupportedEnv;
use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::collections::HashMap;
use std::process::Command;

pub struct ReplHelper {
    external_executables: Vec<Pair>,
    completer: FilenameCompleter,
    built_ins: Vec<Pair>,
}

impl ReplHelper {
    pub(crate) fn new() -> Self {
        ReplHelper {
            external_executables: build_executables_list(),
            completer: FilenameCompleter::new(),
            built_ins: vec![
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
                Pair {
                    display: "history".to_string(),
                    replacement: "history ".to_string(),
                },
            ],
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
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if line[..pos].contains(' ') {
            let line_before_cursor = &line[..pos];
            let tokens: Vec<&str> = line_before_cursor.split_whitespace().collect();

            let command = tokens.first().copied();
            let ends_with_space = line_before_cursor.ends_with(' ');
            let start = if ends_with_space {
                pos
            } else {
                line_before_cursor.rfind(' ').map_or(0usize, |i| i + 1)
            };
            let current_word = if ends_with_space {
                ""
            } else {
                tokens.last().copied().unwrap_or("")
            };
            let previous_word = if ends_with_space {
                tokens.last().copied().unwrap_or("")
            } else {
                tokens
                    .len()
                    .checked_sub(2)
                    .and_then(|index| tokens.get(index))
                    .copied()
                    .unwrap_or("")
            };

            if let Some(command_name) = command {
                if let Some(record) = complete_store::get_all()
                    .into_iter()
                    .find(|record| record.command == command_name)
                {
                    let mut envs: HashMap<String, String> = HashMap::with_capacity(2);

                    envs.insert(String::from("COMP_LINE"), line_before_cursor.to_string());
                    envs.insert(
                        String::from("COMP_POINT"),
                        line_before_cursor.len().to_string(),
                    );

                    if let Ok(output) = Command::new(record.path)
                        .envs(envs)
                        .args(vec![
                            record.command,
                            current_word.to_string(),
                            previous_word.to_string(),
                        ])
                        .output()
                    {
                        if let Ok(stdout) = String::from_utf8(output.stdout) {
                            let candidates: Vec<Pair> = stdout
                                .lines()
                                .filter(|line| !line.is_empty())
                                .map(|line| Pair {
                                    display: line.to_string(),
                                    replacement: format!("{} ", line),
                                })
                                .collect();

                            if !candidates.is_empty() {
                                return Ok((start, candidates));
                            }
                        }
                    }
                }
            }

            let (start, candidates) = self.completer.complete(line, pos, ctx)?;
            let mut candidates: Vec<Pair> = candidates
                .into_iter()
                .map(|c| {
                    let is_dir = c.replacement.ends_with('/');
                    let replacement = if is_dir {
                        c.replacement
                    } else {
                        format!("{} ", c.replacement)
                    };
                    let display = if is_dir {
                        format!("{}/", c.display)
                    } else {
                        c.display
                    };
                    Pair {
                        display,
                        replacement,
                    }
                })
                .collect();
            candidates.sort_by(|a, b| a.display.cmp(&b.display));
            return Ok((start, candidates));
        }

        let candidates: Vec<Pair> = self
            .built_ins
            .clone()
            .into_iter()
            .chain(self.external_executables.clone().into_iter())
            .collect();

        let start = line[..pos].rfind(' ').map_or(0usize, |i| i + 1);
        let prefix = &line[start..pos];

        let matches: Vec<Pair> = candidates
            .into_iter()
            .filter(|c| c.display().starts_with(prefix))
            .collect();

        Ok((start, matches))
    }
}

fn build_executables_list() -> Vec<Pair> {
    let executables: Vec<String> = match std::env::var(SupportedEnv::PATH) {
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
