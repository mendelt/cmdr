//! **Cmdr is a library for building line-oriented text-based user interfaces.**
//!
//! This can be done by implementing one or more objects that implement the Cmdr::Scope trait. A
//! command loop can then be started on a scope with the cmdr::cmd_loop function. The command loop
//! will await user input, parse commands and execute them by running the appropriate functions on
//! the supplied scope object.
//!
//! Implementing a scope is as easy as creating an object with a few methods that take a vector of
//! &str as their input and resturn a CommandResult. By annotating the impl block of that object
//! the cmdr macro all functions starting with do_ in that block will be picked up and transformed
//! into functions.
//!
//! For additional functionality like setting custom prompts or setting hooks to catch unknown or
//! empty commands additional methods can be added to the impl block. These correspond to
//! overridable functions in the Scope trait.

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod line;

pub use crate::line::*;
pub use cmdr_macro::cmdr;

/// Execute a command loop for a scope. This is the main entry point to the cmdr library
/// This function will take commands from the user and try to execute them against the supplied
/// scope until one of the commands returns CommandResult::Quit
pub fn cmd_loop(scope: &mut Scope) -> CommandResult {
    let mut last_result = CommandResult::Ok;

    while last_result == CommandResult::Ok {
        print!("{} ", scope.prompt());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        last_result = scope.command(parse_line(&input));
    }

    last_result
}

/// A command result. returned by one of the client-implemented command methods
#[derive(Debug, PartialEq)]
pub enum CommandResult {
    /// Result Ok, ready to go on to the next command
    Ok,

    /// Result Quit, close the application and stop
    Quit,
}

/// Trait for implementing a Scope object. This trait can be implemented by a client but will most
/// likely be implemented for you by the cmdr macro.
pub trait Scope {
    /// Return the prompt for this scope. The default implementation returns > as a prompt but thus
    /// can be overridden to return other strings or implement dynamically generated prompts
    fn prompt(&self) -> String {
        ">".to_string()
    }

    /// Execute a single line
    fn command(&mut self, line: Line) -> CommandResult;

    /// Execute an empty line.
    /// The default implentation does nothing but this can be overridden by a client-application
    /// to implement other behaviour
    fn empty(&mut self) -> CommandResult {
        CommandResult::Ok
    }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, _line: Line) -> CommandResult {
        println!("Unknown command");
        CommandResult::Ok
    }
}
