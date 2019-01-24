//! Manual implementation of a cmdr application.
//! This example shows how to implement the Scope trait by hand. Normally you'd use the cmdr macro
//! to do the heavy lifting

use cmdr::*;

/// Example Cmdr scope
struct GreeterScope {}

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
    fn command(&mut self, command: CommandLine) -> CommandResult {
        match command.command {
            "greet" => self.do_greet(command.args),
            "quit" => self.do_quit(command.args),
            _ => self.default(command),
        }
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    let mut scope = GreeterScope {};
    scope.cmd_loop();
}
