use crate::state::declare_store;
use crate::syntax::command_invocation::CommandInvocation;
use crate::syntax::pipeline::Pipeline;

pub fn expand_pipeline(pipeline: &mut Pipeline) {
    pipeline
        .commands
        .iter_mut()
        .for_each(expand_command_arguments)
}

fn expand_command_arguments(command: &mut CommandInvocation) {
    for arg in &mut command.arguments {
        *arg = ArgScanner::new(arg.clone()).scan_argument();
    }

    command.arguments.retain(|arg| !arg.is_empty());
}

struct ArgScanner {
    source: Vec<char>,
    current: usize,
    string_builder: String,
}

impl ArgScanner {
    fn new(argument_string: String) -> ArgScanner {
        ArgScanner {
            source: argument_string.chars().collect(),
            current: 0,
            string_builder: String::new(),
        }
    }

    fn scan_argument(mut self) -> String {
        while !self.is_at_end() {
            self.scan_char();
        }

        self.string_builder
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_char(&mut self) {
        let char = self.advance();

        match char {
            '$' => {
                if self.is_at_end() {
                    self.string_builder.push('$');
                } else {
                    self.expand_variable();
                }
            }
            _ => self.string_builder.push(char),
        }
    }

    fn expand_variable(&mut self) {
        let (name_start, name_end, resume_at) = match self.peek() {
            '{' => match self.find_closing_brace() {
                Some(closing_brace_index) => (
                    self.current + 1,
                    closing_brace_index,
                    closing_brace_index + 1,
                ),
                None => {
                    self.string_builder.push('$');
                    return;
                }
            },
            _ => (self.current, self.source.len(), self.source.len()),
        };

        let variable_name: String = self.source[name_start..name_end].iter().collect();

        if let Some(variable_value) = declare_store::get(variable_name.as_str()) {
            self.string_builder.push_str(&variable_value);
        }

        self.current = resume_at;
    }

    fn find_closing_brace(&self) -> Option<usize> {
        if self.peek() != '{' {
            return None;
        }

        ((self.current + 1)..self.source.len()).find(|&index| self.source[index] == '}')
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::expand_pipeline;
    use crate::state::declare_store;
    use crate::syntax::command_invocation::CommandInvocation;
    use crate::syntax::pipeline::Pipeline;

    #[test]
    fn expands_plain_and_braced_variables() {
        let _guard = declare_store::test_lock();
        declare_store::clear();
        declare_store::add("FOO".to_string(), "bar".to_string());
        declare_store::add("BAR".to_string(), "baz".to_string());

        let mut pipeline = Pipeline {
            commands: vec![CommandInvocation {
                name: "echo".to_string(),
                arguments: vec!["$FOO".to_string(), "x${BAR}".to_string()],
                stdout_redirection: None,
                stderr_redirection: None,
            }],
            is_background: false,
        };

        expand_pipeline(&mut pipeline);

        assert_eq!(pipeline.commands[0].arguments, vec!["bar", "xbaz"]);
    }

    #[test]
    fn removes_arguments_that_expand_to_empty() {
        let _guard = declare_store::test_lock();
        declare_store::clear();

        let mut pipeline = Pipeline {
            commands: vec![CommandInvocation {
                name: "echo".to_string(),
                arguments: vec!["$MISSING".to_string(), "kept".to_string()],
                stdout_redirection: None,
                stderr_redirection: None,
            }],
            is_background: false,
        };

        expand_pipeline(&mut pipeline);

        assert_eq!(pipeline.commands[0].arguments, vec!["kept"]);
    }

    #[test]
    fn preserves_dangling_dollar_and_unclosed_braces() {
        let _guard = declare_store::test_lock();
        declare_store::clear();

        let mut pipeline = Pipeline {
            commands: vec![CommandInvocation {
                name: "echo".to_string(),
                arguments: vec!["cost$".to_string(), "${OPEN".to_string()],
                stdout_redirection: None,
                stderr_redirection: None,
            }],
            is_background: false,
        };

        expand_pipeline(&mut pipeline);

        assert_eq!(pipeline.commands[0].arguments, vec!["cost$", "${OPEN"]);
    }
}
