//! #CMDR
//!
//! **Cmdr is a library for building line-oriented text-based user interfaces.**
//!
//! This can be done by implementing one or more objects that implement the Cmdr::Scope trait. These
//! Scope objects can then be run by the Cmdr::cmd_loop executing commands.
//!
//! Implementing the scope trait can be done by hand by implementing the command method and
//! optionally overriding other methods to provide additional functionality. Or you can implement
//! leave the Scope trait up to us and just use the cmdr macro to do the heavy lifting.
//!
//! Any scope implements one or more command methods. Command methods names look like this:
//! ```do_<command>``` where the do_ prefix makes sure the cmdr macro recognizes the function as a
//! command and <command> will be how a user will invoke a command. Command methods take a mutable
//! reference to self (the scope) so they can change the scope when needed, a vector of &str
//! containing any parameters passed on the command line. And they return a CommandResult. This
//! allows the command to specify any follow-up actions to be performed (like quitting the program
//! for example).
//!

use std::io::stdout;
use std::io::stdin;
use std::io::Write;

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

/// A parsed line from the user
pub enum Line<'a> {
    /// An empty line
    Empty,

    /// A user command made up of a command and a series of attributes
    Command(&'a str, Vec<&'a str>)
}

/// A command result. returned by one of the client-implemented command methods
#[derive(PartialEq)]
pub enum CommandResult {
    /// Result Ok, ready to go on to the next command
    Ok,

    /// Result Quit, close the application and stop
    Quit
}

fn parse_line<'a>(line: &'a str) -> Line<'a> {
    let mut split_line = line.trim().split(" ");

    match split_line.next() {
        None => Line::Empty,
        Some(command) => Line::Command(command, split_line.collect()),
    }
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
    fn empty(&mut self) -> CommandResult { CommandResult::Ok }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, _line: Line) -> CommandResult {
        println!("Unknown command");
        CommandResult::Ok
    }
}
