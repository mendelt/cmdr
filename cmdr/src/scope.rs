use crate::description::ScopeDescription;
use crate::line_reader::LineReader;
use crate::result::{CommandError, CommandResult};
use crate::Line;

/// Trait for implementing a Scope object. This trait can be implemented directly but will most
/// likely be implemented for you by the cmdr macro.
pub trait Scope {
    /// Execute commands in this scope. Uses a LineReader to get commands and executes them one by
    /// one until a command returns CommandResult::Quit
    fn run_lines(&mut self, reader: &mut dyn LineReader) -> CommandResult
    where
        Self: Sized,
    {
        self.before_loop();

        let mut last_result = CommandResult::Ok;
        while last_result == CommandResult::Ok {
            last_result = match reader.read_line(self.prompt().as_ref()) {
                Err(error) => CommandResult::Error(error),
                Ok(line_string) => {
                    let line = Line::try_parse(line_string.as_ref());
                    match line {
                        Err(error) => CommandResult::Error(error),
                        Ok(line) => self.run_line(line, reader),
                    }
                }
            };

            if let CommandResult::Error(error) = last_result {
                last_result = self.handle_error_internal(error)
            }
        }

        self.after_loop();

        if last_result == CommandResult::Exit {
            CommandResult::Ok
        } else {
            last_result
        }
    }

    /// Execute a single line
    fn run_line(&mut self, line: Line, reader: &mut dyn LineReader) -> CommandResult
    where
        Self: Sized,
    {
        let line = self.before_command(line);
        let scope_meta = Self::commands();

        let result = if scope_meta.is_help_command(&line.command) {
            self.help(&line.args)
        } else {
            match scope_meta.command_by_name(&line.command) {
                Some(method) => method.execute(self, &line),
                None => self.default(&line),
            }
        };

        let result = if let CommandResult::SubScope(scope_runner) = result {
            scope_runner.run_lines(reader)
        } else {
            result
        };

        self.after_command(&line, result)
    }

    /// Return a ScopeDescription with a set of commands that this scope supports
    fn commands() -> ScopeDescription<Self>
    where
        Self: Sized;

    /// Return the prompt for this scope. The default implementation returns > as the prompt but
    /// this can be overridden to return other strings or implement dynamically generated prompts
    fn prompt(&self) -> String {
        ">".to_string()
    }

    /// Execute a help command
    fn help(&self, args: &[String]) -> CommandResult
    where
        Self: Sized,
    {
        let scope_meta = Self::commands();

        match scope_meta.help(args) {
            Ok(help_text) => {
                println!("\n{}", help_text);
                CommandResult::Ok
            }
            Err(error) => CommandResult::Error(error),
        }
    }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, command_line: &Line) -> CommandResult {
        CommandResult::Error(CommandError::InvalidCommand {
            command: command_line.command.clone(),
        })
    }

    /// Error handling, first allow the user to handle the error, then handles or passes on
    /// unhandled errors
    fn handle_error_internal(&mut self, error: CommandError) -> CommandResult {
        // Allow user to handle error in overridable handle_error
        match self.handle_error(error) {
            CommandResult::Error(error) => {
                // Error was not handled by the user, handle it here
                match error {
                    CommandError::InvalidCommand { command } => {
                        println!("Unknown command: {}", command);
                        CommandResult::Ok
                    }
                    CommandError::InvalidNumberOfArguments { command } => {
                        println!("Invalid number of arguments for command: {}", command);
                        CommandResult::Ok
                    }
                    CommandError::NoHelpForCommand { command } => {
                        println!("No help available for command: {}", command);
                        CommandResult::Ok
                    }
                    CommandError::EmptyLine => CommandResult::Ok,
                    CommandError::CtrlC => CommandResult::Quit,
                    CommandError::CtrlD => CommandResult::Exit,
                    _ => CommandResult::Error(error),
                }
            }
            result => result,
        }
    }

    /// Handle errors, overridable by user
    fn handle_error(&mut self, error: CommandError) -> CommandResult {
        CommandResult::Error(error)
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
