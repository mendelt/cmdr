//! OverrideScope shows how default behavior like prompts, empty command and default command
//! handling can be overridden by implementing prompt, empty and default methods in our Scope

use cmdr::*;

struct ScopeWithHooks {}

/// Example scope that shows how to use the different hooks
#[cmdr]
impl ScopeWithHooks {
    #[cmd]
    fn stuff(&self, _args: &[String]) -> Result<CommandResult, CommandError> {
        println!("Stuff done");

        Ok(CommandResult::Ok)
    }

    fn before_loop(&mut self) {
        println!("This could be a good place to print a hello message for your user")
    }

    fn before_command(&mut self, _line: Line) -> Line {
        println!("Code that gets executed before each command can go here.");
        println!("You can even change what the user typed");

        Line {
            command: "stuff".to_string(),
            args: vec![],
        }
    }

    fn after_command(
        &mut self,
        _line: &Line,
        _result: Result<CommandResult, CommandError>,
    ) -> Result<CommandResult, CommandError> {
        println!("Code that gets executed after each command can go here.");
        println!("For example to change the command result to quit");

        Ok(CommandResult::Quit)
    }

    fn after_loop(&mut self) {
        println!("This is the place where you can put code that gets run when the loop finishes");
        println!("Goodbye!");
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut ScopeWithHooks {}).unwrap();
}
