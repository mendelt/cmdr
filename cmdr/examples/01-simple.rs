//! GreeterScope implements two commands, one greets the user with the supplied name. The other
//! returns CommandResult::Quit to quit the application.

use cmdr::*;

struct GreeterScope {}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone.
    /// Takes one parameter and prints a greeting
    #[cmd(greet)]
    fn greet_method(&self, args: &[String]) -> Result<CommandResult, CommandError> {
        println!("Hello {}", args[0]);
        Ok(CommandResult::Ok)
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd(quit, help = "Quit the application", alias(exit, x, q))]
    fn quit_method(&self, _args: &[String]) -> Result<CommandResult, CommandError> {
        println!("Quitting");
        Ok(CommandResult::Quit)
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {}).unwrap();
}
