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
                Err(error) => self.handle_error_internal(error),
                Ok(line) => self.run_line(line, reader),
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

        let result = if line.command == scope_meta.help_command {
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

        let result = self.after_command(&line, result);

        if let CommandResult::Error(error) = result {
            self.handle_error_internal(error)
        } else {
            result
        }
    }

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
        let scope_metadata = Self::commands();

        if args.len() == 0 {
            if let Some(scope_help) = scope_metadata.get_help() {
                println!("{}", scope_help);
            } else {
                println!("These are the valid commands in this scope:");
            }

            for command in scope_metadata.all_commands() {
                println!("- {}", command.name());
            }

            CommandResult::Ok
        } else if args.len() == 1 {
            match scope_metadata.command_by_name(&args[0]) {
                Some(command) => {
                    if let Some(help_text) = command.help_text() {
                        println!("{}", help_text);
                        CommandResult::Ok
                    } else {
                        return CommandResult::Error(CommandError::NoHelpForCommand {
                            command: command.name().to_string(),
                        });
                    }
                }
                None => {
                    return CommandResult::Error(CommandError::InvalidCommand {
                        command: args[0].clone(),
                    })
                }
            }
        } else {
            CommandResult::Error(CommandError::InvalidNumberOfArguments {
                command: scope_metadata.help_command.clone(),
            })
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

    /// Error handling, first checks if the user handles the error, then
    fn handle_error_internal(&mut self, error: CommandError) -> CommandResult {
        // Allow user to handle error in overridable handle_error
        match self.handle_error(error) {
            CommandResult::Error(error) => {
                // Error was not handled, handle it here

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

/// Metadata describing a scope, is used to return help text and the list of commands that this
/// scope exposes.
pub struct ScopeDescription<T>
where
    T: Scope,
{
    scope_help: Option<String>,
    help_command: String,
    methods: Vec<ScopeCmdDescription<T>>,
}

impl<T> ScopeDescription<T>
where
    T: Scope,
{
    /// Construct a command method list
    pub fn new(
        scope_help: Option<String>,
        help_command: Option<String>,
        methods: Vec<ScopeCmdDescription<T>>,
    ) -> Self {
        ScopeDescription {
            scope_help,
            help_command: help_command.unwrap_or("help".to_string()),
            methods,
        }
    }

    /// Get all scope commands
    pub fn all_commands(&self) -> impl Iterator<Item = &ScopeCmdDescription<T>> {
        self.methods.iter()
    }

    /// Find a command method by its command name or alias
    pub fn command_by_name(&self, name: &str) -> Option<&ScopeCmdDescription<T>> {
        self.methods
            .iter()
            .filter(|method| method.handles(name))
            .next()
    }

    pub fn get_help(&self) -> &Option<String> {
        &self.scope_help
    }
}

/// All information about a command method in one handy struct
pub struct ScopeCmdDescription<T>
where
    T: Scope,
{
    name: String,
    method: Box<dyn Fn(&mut T, &Line) -> CommandResult>,
    alias: Vec<String>,
    help_text: Option<String>,
}

impl<T> ScopeCmdDescription<T>
where
    T: Scope,
{
    /// Construct a CmdMethod from a command name and a command closure
    pub fn new(
        name: String,
        method: Box<dyn Fn(&mut T, &Line) -> CommandResult>,
        alias: Vec<String>,
        help_text: Option<String>,
    ) -> Self {
        ScopeCmdDescription {
            name,
            method,
            alias,
            help_text,
        }
    }

    /// Name accessor method
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Help text accessor method
    pub fn help_text(&self) -> &Option<String> {
        &self.help_text
    }

    /// An iterator of all aliasses for this command
    pub fn aliases(&self) -> impl Iterator<Item = &String> {
        self.alias.iter()
    }

    /// Checks name or alias to see if a command can be handled.
    pub fn handles(&self, command: &str) -> bool {
        if self.name == command {
            true
        } else if self.alias.contains(&command.to_string()) {
            true
        } else {
            false
        }
    }

    /// Execute this command
    pub fn execute(&self, scope: &mut T, command: &Line) -> CommandResult {
        (self.method)(scope, command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestScope {}

    impl TestScope {
        fn test_method(&self, _args: &[String]) -> CommandResult {
            CommandResult::Ok
        }
    }

    impl Scope for TestScope {
        fn commands() -> ScopeDescription<Self> {
            unimplemented!()
        }
    }

    fn get_test_command() -> ScopeCmdDescription<TestScope> {
        ScopeCmdDescription::new(
            "test".to_string(),
            Box::new(|scope, cmd_line| scope.test_method(&cmd_line.args)),
            vec!["alias1".to_string(), "alias2".to_string()],
            Some("Help text\nMore lines".to_string()),
        )
    }

    #[test]
    fn command_should_not_handle_unknown() {
        let command = get_test_command();
        assert!(!command.handles("not_a_command"));
    }

    #[test]
    fn command_should_return_name() {
        let command = get_test_command();

        assert_eq!(command.name(), "test")
    }

    #[test]
    fn command_should_return_help_text() {
        let command = get_test_command();

        assert_eq!(
            command.help_text(),
            &Some("Help text\nMore lines".to_string())
        )
    }

    #[test]
    fn command_should_return_all_aliases() {
        let command = get_test_command();
        let aliasses: Vec<&String> = command.aliases().collect();

        assert_eq!(aliasses, vec!["alias1", "alias2"])
    }

    #[test]
    fn command_should_handle_by_name() {
        let command = get_test_command();

        assert!(command.handles("test"));
    }

    #[test]
    fn command_should_handle_by_alias() {
        let command = get_test_command();

        assert!(command.handles("alias1"));
        assert!(command.handles("alias2"));
    }
}
