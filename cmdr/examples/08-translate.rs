//! Shows how to replace all strings that are output by cmdr with translated versions. This example
//! provides Dutch translations for all messages built into cmdr. And changes the help command to
//! German.

use cmdr::*;

struct TranslatedScope {}

#[cmdr(help_command = "?")]
impl TranslatedScope {
    /// Handle errors, output a translated error string for all known errors
    fn handle_error(&mut self, error: Error) -> CommandResult {
        match error {
            Error::InvalidCommand(command) => {
                println!("Onbekend commando: {}", command);
                Ok(Action::Done)
            }
            Error::InvalidNumberOfArguments(command) => {
                println!("Verkeerd aantal argumenten voor commando: {}", command);
                Ok(Action::Done)
            }
            Error::NoHelpForCommand(command) => {
                println!("Geen hulp beschikbaar voor commando: {}", command);
                Ok(Action::Done)
            }
            _ => Err(error),
        }
    }
}

fn main() -> cmdr::Result<()> {
    cmd_loop(&mut TranslatedScope {})?;
    Ok(())
}
