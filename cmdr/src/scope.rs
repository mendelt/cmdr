use crate::description::ScopeDescription;
use crate::line_reader::LineReader;
use crate::line_writer::LineWriter;
use crate::result::{Action, CommandResult, Error};
use crate::{Line, ScopeCmdDescription};

/// Trait for implementing a Scope object. This trait can be implemented directly but will most
/// likely be implemented for you by the cmdr macro.
pub trait Scope {
    /// Execute commands in this scope. Uses a LineReader to get commands and executes them one by
    /// one until a command returns CommandResult::Quit
    fn run_lines(
        &mut self,
        reader: &mut dyn LineReader,
        writer: &mut dyn LineWriter,
    ) -> CommandResult {
        self.before_loop();

        let mut last_result = Ok(Action::Done);
        let commands = self.commands();

        while last_result == Ok(Action::Done) {
            last_result = match reader.read_line(self.prompt().as_ref()) {
                Err(error) => CommandResult::Err(error),
                Ok(line_string) => {
                    let line = Line::try_parse(line_string.as_ref());
                    match line {
                        Err(error) => CommandResult::Err(error),
                        Ok(line) => {
                            let line = self.before_command(line);

                            let result = match commands.command_for_line(&line) {
                                Some(command) => self.run_command(&command, &line.args, writer),
                                None => self.default(&line),
                            };

                            let result =
                                if let CommandResult::Ok(Action::SubScope(mut scope_runner)) =
                                    result
                                {
                                    scope_runner.run_lines(reader, writer)
                                } else {
                                    result
                                };

                            self.after_command(&line, result)
                        }
                    }
                }
            };

            if let CommandResult::Err(error) = last_result {
                last_result = self.handle_error_internal(error)
            }
        }

        self.after_loop();

        match last_result {
            CommandResult::Ok(Action::Exit) => Ok(Action::Done),
            _ => last_result,
        }
    }

    /// Return a ScopeDescription with a set of commands that this scope supports. This is used by
    /// the help function and maybe in the future by tab completion.
    fn commands(&self) -> ScopeDescription;

    /// Run an entered command and return the result
    fn run_command(
        &mut self,
        command: &ScopeCmdDescription,
        args: &[String],
        writer: &mut dyn LineWriter,
    ) -> CommandResult;

    /// Return the prompt for this scope. The default implementation returns > as the prompt but
    /// this can be overridden to return other strings or implement dynamically generated prompts
    fn prompt(&self) -> String {
        ">".to_string()
    }

    /// Execute a help command
    fn help(&self, args: &[String]) -> CommandResult {
        let command = args.get(0).map(|stuff| stuff.as_ref());
        let help_text = self.commands().format_help_text(command)?;

        println!("\n{}", help_text);
        Ok(Action::Done)
    }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, command_line: &Line) -> CommandResult {
        CommandResult::Err(Error::InvalidCommand(command_line.command.clone()))
    }

    /// Error handling, first allow the user to handle the error, then handles or passes on
    /// unhandled errors
    fn handle_error_internal(&mut self, error: Error) -> CommandResult {
        // Allow user to handle error in overridable handle_error
        match self.handle_error(error) {
            CommandResult::Err(error) => {
                // Error was not handled by the user, handle it here
                match error {
                    Error::InvalidCommand(command) => {
                        println!("Unknown command: {}", command);
                        Ok(Action::Done)
                    }
                    Error::InvalidNumberOfArguments(command) => {
                        println!("Invalid number of arguments for command: {}", command);
                        Ok(Action::Done)
                    }
                    Error::NoHelpForCommand(command) => {
                        println!("No help available for command: {}", command);
                        Ok(Action::Done)
                    }
                    Error::EmptyLine => Ok(Action::Done),
                    Error::CtrlC => Ok(Action::Done),
                    Error::CtrlD => Ok(Action::Done),
                    _ => CommandResult::Err(error),
                }
            }
            result => result,
        }
    }

    /// Handle errors, overridable by user
    fn handle_error(&mut self, error: Error) -> CommandResult {
        CommandResult::Err(error)
    }

    /// Hook that is called before the command loop starts, can be overridden
    fn before_loop(&mut self) {}

    /// Hook that is called before executing a command, can be overridden
    fn before_command(&mut self, line: Line) -> Line {
        line
    }

    /// Hook that is called after command execution is finished, can be overridden
    fn after_command(&mut self, _line: &Line, result: CommandResult) -> CommandResult {
        result
    }

    /// Hook that is called after the command loop finishes, can be overridden
    fn after_loop(&mut self) {}
}
