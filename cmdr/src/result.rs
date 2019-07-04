use crate::line_reader::LineReader;
use crate::Scope;
use std::fmt::{Debug, Error, Formatter};
use std::ptr;

/// A command result. returned by one of the client-implemented command methods
#[derive(Debug, PartialEq)]
pub enum CommandResult {
    /// Result Ok, ready to go on to the next command
    Ok,

    /// Switch to a new scope
    NewScope(ScopeRunner),

    /// Switch to a sub scope,
    SubScope(ScopeRunner),

    /// Result Exit, exit the current scope and return to the parent scope if available
    Exit,

    /// Result Quit, close the application and stop
    Quit,

    /// Error
    Error(CommandError),
}

impl CommandResult {
    /// Construct a CommandResult::NewScope around the provided scope
    pub fn new_scope<S: Scope + Sized + 'static>(scope: S) -> Self {
        CommandResult::NewScope(ScopeRunner::new(scope))
    }

    /// Construct a CommandResult::SubScope around the provided scope
    pub fn sub_scope<S: Scope + Sized + 'static>(scope: S) -> Self {
        CommandResult::SubScope(ScopeRunner::new(scope))
    }
}

/// Specifies an error while parsing or executing a command
#[derive(Debug, PartialEq)]
pub enum CommandError {
    /// Invalid command was entered
    InvalidCommand { command: String },

    /// Invalid numer of arguments
    InvalidNumberOfArguments { command: String },

    /// No help for the entered command
    NoHelpForCommand { command: String },

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

/// Return the new scope to start on a CommandResult::NewScope
pub struct ScopeRunner {
    runner: Box<dyn (FnOnce(&mut LineReader) -> CommandResult)>,
}

impl ScopeRunner {
    pub fn new<S: Sized + Scope + 'static>(mut scope: S) -> Self {
        ScopeRunner {
            runner: Box::new(move |reader| scope.run_lines(reader)),
        }
    }

    pub fn run_lines(self, reader: &mut LineReader) -> CommandResult {
        (self.runner)(reader)
    }
}

/// Do not attempt to print anything about the ScopeRunner, just show that this is a ScopeRunner
impl Debug for ScopeRunner {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "NewScopeResult")
    }
}

/// Different instances of ScopeRunner are never equal
impl PartialEq for ScopeRunner {
    fn eq(&self, other: &ScopeRunner) -> bool {
        ptr::eq(self, other)
    }
}
