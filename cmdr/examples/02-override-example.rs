//! OverrideScope shows how default behavior like prompts, empty command and default command
//! handling can be overridden by implementing prompt, empty and default methods in our Scope

use cmdr::*;

struct OverrideScope {}

/// Example scope that overrides prompt
/// TODO: override empty and default
/// TODO: remove do_ methods
#[cmdr]
impl OverrideScope {
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

    /// I reject your prompt and substitute my own
    fn prompt(&self) -> String {
        "#".to_string()
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    let mut scope = OverrideScope {};
    cmd_loop(&mut scope);
}
