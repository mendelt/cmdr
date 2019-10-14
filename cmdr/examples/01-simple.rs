//! GreeterScope implements two commands, one greets the user with the supplied name. The other
//! returns CommandResult::Quit to quit the application.

use cmdr::*;

struct GreeterScope {}

/// Example scope that implements two commands, greet and quit, help is available with the new_help
/// command
#[cmdr(help_command = "new_help", help = "Example scope")]
impl GreeterScope {
    /// Cmdr command to greet someone.
    /// Takes one parameter and prints a greeting
    #[cmd(greet)]
    fn greet_method(&self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd(quit, help = "Quit the application", alias(exit, x, q))]
    fn quit_method(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {});
}
