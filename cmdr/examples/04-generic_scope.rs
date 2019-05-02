//! Example implementation of a generic version of the greeterscope from example 01. Currently
//! you can't use the #[cmdr] macro on generic types so this example shows a workaround by
//! implementing the scope trait by hand (see issue 36 [https://github.com/mendelt/cmdr/issues/36])

use cmdr::*;

struct GreeterScope<T, G>
where
    T: PartialEq,
{
    _generic_t_member: T,
    _generic_g_member: G,
}

/// Example scope that implements two commands, greet and quit
// #[cmdr]
impl<T, G> GreeterScope<T, G>
where
    T: PartialEq,
{
    /// Cmdr command to greet someone.
    /// Takes one parameter and prints a greeting

    // #[cmd]
    fn greet(&mut self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    // #[cmd]
    fn quit(&mut self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

impl<T, G> Scope for GreeterScope<T, G>
where
    T: PartialEq,
{
    fn commands() -> ScopeDescription<GreeterScope<T, G>> {
        ScopeDescription::new(
            Some("Manual greeter scope".to_string()),
            vec![
                ScopeCmdDescription::new(
                    "greet".to_string(),
                    Box::new(|scope, cmd_line| scope.greet(&cmd_line.args)),
                    Vec::new(),
                    Some("Show a greeting.".to_string()),
                ),
                ScopeCmdDescription::new(
                    "quit".to_string(),
                    Box::new(|scope, cmd_line| scope.quit(&cmd_line.args)),
                    Vec::new(),
                    Some("Quit the application.".to_string()),
                ),
            ],
        )
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {
        _generic_t_member: "String T".to_string(),
        _generic_g_member: "String G".to_string(),
    });
}
