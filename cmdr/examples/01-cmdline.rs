//! Example for the cmdr crate.
//!
//! Implements a simple cmdr application that does not use the cmdr macro.
//! It provides two commands, One that gives a greeting to someone depending on the supplied
//!   parameter and one to quit the application

use cmdr::*;

/// Example Cmdr scope
struct GreeterScope { }

impl GreeterScope {
    /// Cmdr command to greet someone.
    pub fn do_greet(&self, args: Vec<&str>) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    pub fn do_quit(&self, _args: Vec<&str>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Manual scope implementation for Cmdr. Normally you'd use the cmdr macro for this. Implements
/// the command method that dispatches commands to functions implemented above.
impl Scope for GreeterScope {
    fn command(&mut self, line: Line) -> CommandResult {
        match line {
            Line::Empty => self.empty(),
            Line::Command("greet", args) => self.do_greet(args),
            Line::Command("quit", args) => self.do_quit(args),
            _ => self.default(line)
        }
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main(){
    let mut scope = GreeterScope {};
    cmd_loop(&mut scope);
}
