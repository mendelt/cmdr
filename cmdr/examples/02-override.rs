//! OverrideScope shows how default behavior like prompts, empty command and default command
//! handling can be overridden by implementing prompt, empty and default methods in our Scope

use cmdr::*;

struct OverrideScope {}

/// Example scope that overrides prompt
/// TODO: override empty and default
/// TODO: remove do_ methods
#[cmdr]
impl OverrideScope {
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
