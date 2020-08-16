use crate::Scope;
use std::fmt::Debug;
use std::result::Result as StdResult;

/// Default cmdr Result type
pub type Result<T> = StdResult<T, Error>;

/// Result type for returning from Command
pub type CommandResult = Result<Action>;

/// Returned by one of the client-implemented command methods to indicate what needs to happen next
pub enum Action {
    /// Result Ok, ready to go on to the next command
    Done,

    /// Switch to a new scope
    NewScope(Box<dyn Scope>),

    /// Switch to a sub scope,
    SubScope(Box<dyn Scope>),

    /// Result Exit, exit the current scope and return to the parent scope if available
    Exit,

    /// Result Quit, close the application and stop
    Quit,
}

impl Debug for Action {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Action::Done => formatter.debug_tuple("Done").finish(),
            Action::NewScope(_) => formatter.debug_tuple("NewScope").finish(),
            Action::SubScope(_) => formatter.debug_tuple("SubScope").finish(),
            Action::Exit => formatter.debug_tuple("Exit").finish(),
            Action::Quit => formatter.debug_tuple("Quit").finish(),
        }
    }
}

impl Action {
    /// Shortcut to construct a NewScope action to return from a command
    /// This ends the current scope and starts a new scope
    pub fn new_scope<S: Scope + 'static>(scope: S) -> CommandResult {
        CommandResult::Ok(Action::NewScope(Box::new(scope)))
    }

    /// Shortcut to construct a SubScope action to return from a command
    /// This recursively starts a subscope that will return to the current scope when done
    pub fn sub_scope<S: Scope + 'static>(scope: S) -> CommandResult {
        CommandResult::Ok(Action::SubScope(Box::new(scope)))
    }
}

/// Specifies an error while parsing or executing a command
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid command was entered
    InvalidCommand(String),

    /// Invalid number of arguments
    InvalidNumberOfArguments(String),

    /// No help for the entered command
    NoHelpForCommand(String),

    /// An unknown error occured reading a line
    LineReaderError,

    /// An empty line was read
    EmptyLine,

    /// Control C was pressed
    CtrlC,

    /// Control D was pressed
    CtrlD,

    /// Fatal error, quit the application with an error code
    Fatal(i32),
}
