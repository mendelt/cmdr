//! Manual implementation of a cmdr application.
//! This example shows how to implement the Scope trait by hand. Normally you'd use the cmdr macro
//! to do the heavy lifting. But you can use cmdr without using macro's too.

use cmdr::*;

/// Example Cmdr scope
struct GreeterScope {}

impl GreeterScope {
    /// Cmdr command to greet someone.
    fn do_greet(&self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    fn do_quit(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Manual scope implementation for Cmdr. Normally you'd use the cmdr macro for this. Implements
/// the command method that dispatches commands to functions implemented above.
impl Scope for GreeterScope {
    fn commands() -> ScopeDescription<GreeterScope> {
        ScopeDescription::new(
            Some("Manual greeter scope".to_string()),
            vec![
                ScopeCmdDescription::new(
                    "greet".to_string(),
                    Box::new(|scope, cmd_line| scope.do_greet(&cmd_line.args)),
                    Some("Show a greeting.".to_string()),
                ),
                ScopeCmdDescription::new(
                    "quit".to_string(),
                    Box::new(|scope, cmd_line| scope.do_quit(&cmd_line.args)),
                    Some("Quit the application.".to_string()),
                ),
            ],
        )
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {});
}
