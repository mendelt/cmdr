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

The code is pretty straight forward. We define a type to keep the state for our application. Cmdr
calls this the `scope` and it is used to keep the state during execution of this part of the
application. You can have multiple scopes during the lifetime of the application if you want.

The `scope` can be any type you define.

A type can be made into a `scope` by decorating an impl block on it with the `#[cmdr]` macro. This
will implement the `Scope` trait for this type so it can be run. The '#[cmdr]' macro works similarly
to derive macro's on structs and enums. But because it works on methods implemented on a type it
should be used to decorate an impl block instead of the main type.

Commands can be defined on a `scope` by implementing methods in the scope's impl block. And
decorating them with `#[cmd]`. 
These methods need to take a reference or mutable reference to `self` and a reference to a String slice.
And they need to return a `CommandResult`. Just like the `greet` command in our example;
```rust
#[cmd]`
fn greet_method(&self, args: &[String]) -> CommandResult {
```
The arguments to the command will be passed as Strings in the args string slice so they can be
parsed by the command. The command can specify one of a set of things to happen after the command
by returning different CommandResults. Usually this will be ``CommandResult::Ok` But the `quit` 
command for example tells cmdr to quit the application by returning `CommandResult::Quit`

Now all that is left for us to do is to start our `GreeterScope` and start handling commands.
We can do this in the main function;
```rust
fn main() {
    cmd_loop(&mut GreeterScope {});
}
```
