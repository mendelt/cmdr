# Getting started

Cmdr is a library for writing simple interactive command line applications. Instead of trying to
explain what I mean by that, let's just write one.

Start a new project;
```
> cargo new my_cli_app
```

Add a dependency to cmdr to Cargo.toml;
```
[dependencies]
cmdr = "0.3.11"
```

and enter the following code in `src/main.rs`

```rust
use cmdr::*;

/// Example scope that implements two commands, greet and quit
struct GreeterScope {}

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

If you run this code with
```
> cargo run
```
It will present you with a command prompt `>` and a blinking cursor. Typing

```
> help
```

will tell you what commands are available. You can greet someone by typing `greet` followed by the
name of the person you'd like to extend a greeting to. You can exit the application by typing
`quit`.
