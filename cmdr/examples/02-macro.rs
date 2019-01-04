//! Example for the cmdr crate.
//!
//! Same as the first example but this uses the cmdr macro.

use cmdr::*;

/// Example Cmdr scope
struct GreeterScope { }



#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone.
    pub fn do_greet(&self, args: Vec<&str>) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Succes
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    pub fn do_quit(&self, _args: Vec<&str>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main(){
    let mut scope = GreeterScope {};
    cmd_loop(&mut scope);
}
