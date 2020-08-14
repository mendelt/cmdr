# Getting started

Cmdr is a library for writing simple interactive command line applications. Instead of trying to
explain what I mean by that, let's just write one.

Start a new project;
```
> cargo new my_cli_app
```

and enter the following code in `src/main.rs`

```rust
use cmdr::*;
struct GreeterScope {}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone. Takes one parameter and prints a greeting
    #[cmd]
    fn greet(&self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd]
    fn quit(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {});
}
```
