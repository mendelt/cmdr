//! Shows how to replace all strings that are output by cmdr with translated versions. This example
//! provides Dutch translations for all messages built into cmdr.
//!
//! This translates;
//! - Error messages by handling user_error
//! -

use cmdr::*;

struct TranslatedScope {}

#[cmdr]
impl TranslatedScope {
    /// Handle errors, output a translated error string for all known errors
    fn handle_error(&mut self, error: CommandError) -> CommandResult {
        match error {
            CommandError::InvalidCommand { command } => {
                println!("Onbekend commando: {}", command);
                CommandResult::Ok
            }
            CommandError::InvalidNumberOfArguments { command } => {
                println!("Verkeerd aantal argumenten voor commando: {}", command);
                CommandResult::Ok
            }
            CommandError::NoHelpForCommand { command } => {
                println!("Geen hulp beschikbaar voor commando: {}", command);
                CommandResult::Ok
            }
            _ => CommandResult::Error(error),
        }
    }
}

fn main() {
    cmd_loop(&mut TranslatedScope {});
}
