//! GreeterScope implements two commands, one greets the user with the supplied name. The other
//! returns CommandResult::Quit to quit the application.

use cmdr::*;

struct GreeterScope { }

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone. Takes one parameter and prints a greeting
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

/// Main function that creates the scope and starts a command loop for it
fn main(){
    let mut scope = GreeterScope {};
    cmd_loop(&mut scope);
}
