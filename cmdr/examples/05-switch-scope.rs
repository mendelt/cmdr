//! This example shows how to use commandresults to switch between scopes. You can switch to
//! a different instance of the same scope type (for example a directory scope for a different
//! directory) or you can switch to a scope of a different type to have a new set of commands
//! available.

use cmdr::*;

struct FirstScope {}

#[cmdr]
impl FirstScope {
    fn prompt(&self) -> String {
        "first scope>".to_string()
    }

    #[cmd]
    /// Switch to the second scope
    fn switch(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::new_scope(SecondScope {})
    }
}

struct SecondScope {}

#[cmdr]
impl SecondScope {
    fn prompt(&self) -> String {
        "second scope>".to_string()
    }

    #[cmd]
    fn switch_back(&mut self, _args: &[String]) -> CommandResult {
        CommandResult::new_scope(FirstScope {})
    }
}

fn main() {
    cmd_loop(&mut FirstScope {});
}
