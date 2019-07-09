//! Example implementation of a scope on a generic struct.

use cmdr::*;

struct GreeterScope<T, G>
where
    T: PartialEq,
{
    _generic_t_member: T,
    _generic_g_member: G,
}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl<T, G> GreeterScope<T, G>
where
    T: PartialEq,
{
    /// Cmdr command to greet someone.
    /// Takes one parameter and prints a greeting

    #[cmd]
    fn greet(&mut self, args: &[String]) -> Result<CommandResult, CommandError> {
        println!("Hello {}", args[0]);
        Ok(CommandResult::Ok)
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd]
    fn quit(&mut self, _args: &[String]) -> Result<CommandResult, CommandError> {
        println!("Quitting");
        Ok(CommandResult::Quit)
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {
        _generic_t_member: "String T".to_string(),
        _generic_g_member: "String G".to_string(),
    })
    .unwrap();
}
