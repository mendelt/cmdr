//! Return errors from commands, right now we only have the fatal error that exits the application
//! it allows you to return an error code that can be returned from main()
//! CommandResult will probably be re-factored in version 0.4.0 to be a normal Result<> Type that
//! can either be a CommandResult or a CommandError

use cmdr::*;

struct MainScope {}

#[cmdr]
impl MainScope {
    fn prompt(&self) -> String {
        "main scope>".to_string()
    }

    #[cmd]
    /// Return a fatal error to quit the application with an error code
    fn error(&mut self, _args: &[String]) -> CommandResult {
        Err(Error::Fatal(101))
    }
}

fn main() {
    std::process::exit(match cmd_loop(&mut MainScope {}) {
        Ok(_) => 0,
        Err(Error::Fatal(error_code)) => error_code,
        _ => -1, // This should not happen.
    })
}
