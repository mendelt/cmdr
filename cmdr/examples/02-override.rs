//! OverrideScope shows how default behavior like prompts, empty command and default command
//! handling can be overridden by implementing prompt, empty and default methods in our Scope

use cmdr::*;

struct OverrideScope {}

/// Example scope that demonstrates overriding prompt, empty and default
#[cmdr]
impl OverrideScope {
    /// I reject your prompt and substitute my own
    fn prompt(&self) -> String {
        "#".to_string()
    }

    /// Passive agressive empty line handler override
    fn empty(&self) -> CommandResult {
        println!("If you don't want to talk to me I'll just go then...");

        CommandResult::Quit
    }

    /// Default line handler override
    fn default(&mut self, command: &CommandLine) -> CommandResult {
        println!("{}? What does that even mean?", command.command);

        CommandResult::Ok
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    let mut scope = OverrideScope {};
    scope.cmd_loop();
}
