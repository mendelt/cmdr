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

    /// All the help, all the time
    fn help(&self, _args: &[String]) -> CommandResult {
        let scope_metadata = Self::commands();

        println!("Help Stuff");
        for command in scope_metadata.all_commands() {
            println!("- {}", command.name());
            if let Some(help_text) = &command.help_text() {
                println!("{}", help_text)
            }
        }

        CommandResult::Ok
    }

    /// Handle empty line here, pass other error up to be handled by cmdr
    fn handle_error(&mut self, error: CommandError) -> CommandResult {
        if let CommandError::EmptyLine = error {
            println!("Quitting because empty");
            CommandResult::Quit
        } else {
            CommandResult::Error(error)
        }
    }

    /// Default line handler override
    fn default(&mut self, command: &Line) -> CommandResult {
        println!("{}? What does that even mean?", command.command);

        CommandResult::Ok
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut OverrideScope {});
}
