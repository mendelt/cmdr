use crate::line_reader::LineReader;
use crate::Scope;
use std::fmt::{Debug, Error as StdError, Formatter};
use std::ptr;
use std::result::Result as StdResult;

/// Default cmdr Result type
pub type Result<T> = StdResult<T, Error>;

/// Result type for returning from Command
pub type CommandResult = Result<Action>;

/// Returned by one of the client-implemented command methods to indicate what needs to happen next
#[derive(Debug, PartialEq)]
pub enum Action {
    /// Result Ok, ready to go on to the next command
    Done,

    /// Switch to a new scope
    NewScope(ScopeWrap),

    /// Switch to a sub scope,
    SubScope(ScopeWrap),

    /// Result Exit, exit the current scope and return to the parent scope if available
    Exit,

    /// Result Quit, close the application and stop
    Quit,
}

impl Action {
    /// Shortcut to construct a NewScope action to return from a command
    /// This ends the current scope and starts a new scope
    pub fn new_scope<S: Scope + Sized + 'static>(scope: S) -> CommandResult {
        CommandResult::Ok(Action::NewScope(ScopeWrap::new(scope)))
    }

    /// Shortcut to construct a SubScope action to return from a command
    /// This recursively starts a subscope that will return to the current scope when done
    pub fn sub_scope<S: Scope + Sized + 'static>(scope: S) -> CommandResult {
        CommandResult::Ok(Action::SubScope(ScopeWrap::new(scope)))
    }
}

/// Specifies an error while parsing or executing a command
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid command was entered
    InvalidCommand {
        /// The command-string that caused the error
        command: String,
    },

    /// Invalid number of arguments
    InvalidNumberOfArguments {
        /// The command-string that caused the error
        command: String,
    },

    /// No help for the entered command
    NoHelpForCommand {
        /// The command-string that caused the error
        command: String,
    },

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

// impl Error {
//     /// Shortcut for constructing an invalid command error result
//     pub fn invalid_command(command: String) -> CommandResult {
//         Err(Error::InvalidCommand{command})
//     }

//     /// Shortcut for constructing an invalid command error result
//     pub fn invalid_num_arguments(command: String) -> CommandResult {
//         CommandResult::Err(Error::InvalidNumberOfArguments{command})
//     }

//     /// Shortcut for constructing a no help for command error result
//     pub fn no_help(command: String) -> CommandResult {
//         CommandResult::Err(Error::NoHelpForCommand{command})
//     }

//     /// Shortcut for constructing a linereader error result
//     pub fn linereader() -> CommandResult {
//         Err(Error::LineReaderError)
//     }

//     /// Shortcut for constructing an empty line error result
//     pub fn empty_line() -> CommandResult {
//         Err(Error::EmptyLine)
//     }

//     /// Shortcut for constructing a ctrl c error result
//     pub fn ctrl_c() -> CommandResult {
//         Err(Error::CtrlC)
//     }

//     /// Shortcut for constructing a ctrl d error result
//     pub fn ctrl_d() -> CommandResult {
//         Err(Error::CtrlD)
//     }

//     /// Shortcut for constructing a ctrl d error result
//     pub fn fatal(code: i32) -> CommandResult {
//         CommandResult::Err(Error::Fatal(code))
//     }
// }

/// Wrap the scope to start on a CommandResult::NewScope or CommandResult::SubScope
pub struct ScopeWrap {
    runner: Box<dyn (FnOnce(&mut dyn LineReader) -> CommandResult)>,
}

impl ScopeWrap {
    pub fn new<S: Sized + Scope + 'static>(mut scope: S) -> Self {
        ScopeWrap {
            runner: Box::new(move |reader| scope.run_lines(reader)),
        }
    }

    pub fn run_lines(self, reader: &mut dyn LineReader) -> CommandResult {
        (self.runner)(reader)
    }
}

/// Do not attempt to print anything about the ScopeRunner, just show that this is a ScopeRunner
impl Debug for ScopeWrap {
    fn fmt(&self, formatter: &mut Formatter) -> StdResult<(), StdError> {
        write!(formatter, "NewScopeResult")
    }
}

/// Different instances of ScopeRunner are never equal
impl PartialEq for ScopeWrap {
    fn eq(&self, other: &ScopeWrap) -> bool {
        ptr::eq(self, other)
    }
}
