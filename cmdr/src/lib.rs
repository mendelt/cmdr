//! **Cmdr is a library for building line-oriented text-based user interfaces.**
//!
//! This can be done by implementing one or more objects that implement the Cmdr::Scope trait. A
//! command loop can then be started on a scope by calling the cmd_loop method. The command loop
//! will await user input, parse commands and execute them by running the appropriate functions on
//! the supplied scope object.
//!
//! Implementing a scope is as easy as creating an object with a few methods that take a vector of
//! &str as their input and return a CommandResult. By annotating the impl block of that object
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
    /// Execute a command loop for a scope. This is the main entry point to the cmdr library
    /// This method will take commands from the user and execute them until one of the commands
    /// returns CommandResult::Quit
    fn cmd_loop(&mut self) -> CommandResult {
        let mut last_result = CommandResult::Ok;

        while last_result == CommandResult::Ok {
            print!("{} ", self.prompt());
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            last_result = self.one_line(input[..].into());
        }

        last_result
    }

    /// Execute a single line
    fn one_line(&mut self, line: Line) -> CommandResult {
        match line {
            Line::Empty => self.empty(),
            Line::Command(command) => self.command(command),
        }
    }

    /// Execute a single command, must be implemented by trait implementors or by the cmdr macro
    fn command(&mut self, command: CommandLine) -> CommandResult;

    /// Return the prompt for this scope. The default implementation returns > as the prompt but
    /// this can be overridden to return other strings or implement dynamically generated prompts
    fn prompt(&self) -> String {
        ">".to_string()
    }

    /// Execute an empty line.
    /// The default implentation does nothing but this can be overridden by a client-application
    /// to implement other behaviour
    fn empty(&mut self) -> CommandResult {
        CommandResult::Ok
    }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, _command: CommandLine) -> CommandResult {
        println!("Unknown command");
        CommandResult::Ok
    }
}
