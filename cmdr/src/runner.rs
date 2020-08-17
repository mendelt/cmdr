use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::Line;
use crate::{line_writer::LineWriter, result::Action, CommandResult};
use std::fmt::Debug;

/// Wraps a LineReader and a Scope and allows using the scope to interpret commands from the
/// LineReader
#[derive(Debug)]
pub struct Runner<R: LineReader, W: LineWriter> {
    reader: R,
    writer: W,
}

impl<R: LineReader, W: LineWriter> Runner<R, W> {
    /// Create a new runner that takes lines from the `reader` and executes them using the `scope`
    pub fn new(reader: R, writer: W) -> Self {
        Runner { reader, writer }
    }

    /// Start reading lines and executing them
    pub fn run<S: Scope>(&mut self, scope: &mut S) -> CommandResult {
        let mut result = self.run_scope(scope);

        while let CommandResult::Ok(Action::NewScope(mut sub_scope)) = result {
            result = self.run_scope(sub_scope.as_mut());
        }

        result
    }

    /// Execute commands in this scope. Uses a LineReader to get commands and executes them one by
    /// one until a command returns CommandResult::Quit
    fn run_scope(&mut self, scope: &mut dyn Scope) -> CommandResult {
        scope.before_loop();

        let mut last_result = Ok(Action::Done);
        let commands = scope.commands();

        while let Ok(Action::Done) = last_result {
            last_result = match self.reader.read_line(scope.prompt().as_ref()) {
                Err(error) => CommandResult::Err(error),
                Ok(line_string) => {
                    let line = Line::try_parse(line_string.as_ref());
                    match line {
                        Err(error) => CommandResult::Err(error),
                        Ok(line) => {
                            let line = scope.before_command(line);

                            let result = match commands.command_for_line(&line) {
                                Some(command) => {
                                    scope.run_command(&command, &line.args, &mut self.writer)
                                }
                                None => scope.default(&line),
                            };

                            let result = if let CommandResult::Ok(Action::SubScope(mut sub_scope)) =
                                result
                            {
                                self.run_scope(sub_scope.as_mut())
                            } else {
                                result
                            };

                            scope.after_command(&line, result)
                        }
                    }
                }
            };

            if let CommandResult::Err(error) = last_result {
                last_result = scope.handle_error_internal(error)
            }
        }

        scope.after_loop();

        match last_result {
            CommandResult::Ok(Action::Exit) => Ok(Action::Done),
            _ => last_result,
        }
    }
}
