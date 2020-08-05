use crate::line::Line;
use crate::result::CommandError;
use crate::result::CommandResult;
use crate::scope::Scope;
use std::fmt::{Debug, Error as FmtError, Formatter};

/// Metadata describing a scope, is used to return help text and the list of commands that this
/// scope exposes.
#[derive(Debug)]
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

    pub fn is_help_command(&self, command: &str) -> bool {
        self.help_command == command
    }

    /// Format help text for command
    pub fn help(&self, args: &[String]) -> Result<String, CommandError> {
        match args.len() {
            0 => Ok(self.format_scope_help()),
            1 => match self
                .command_by_name(&args[0])
                .ok_or(CommandError::InvalidCommand {
                    command: args[0].to_owned(),
                })?
                .help_text
                .clone()
            {
                Some(help_text) => Ok(help_text),
                None => Err(CommandError::NoHelpForCommand {
                    command: args[0].to_owned(),
                }),
            },
            _ => Err(CommandError::InvalidNumberOfArguments {
                command: self.help_command.clone(),
            }),
        }
    }

    fn format_scope_help(&self) -> String {
        let mut result = String::new();

        result.push_str(match &self.scope_help {
            Some(scope_help) => scope_help,
            None => "These are the valid commands in this scope:",
        });

        result.push('\n');

        for command in self.all_commands() {
            result = format!("{}- {}\n", result, command.name());
        }

        result
    }
}

/// All information about a command method in one handy struct
pub struct ScopeCmdDescription<T> {
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

impl<T> Debug for ScopeCmdDescription<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), FmtError> {
        formatter
            .debug_struct("ScopeCmdDescription")
            .field("name", &self.name)
            .field("alias", &self.alias)
            .field("help_text", &self.help_text)
            .finish()
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
