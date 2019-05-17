# cmdr &emsp; [![Build Status](https://travis-ci.org/mendelt/cmdr.svg?branch=master)](https://travis-ci.org/mendelt/cmdr)

**Cmdr is a library for building line-oriented text-based user interfaces.**
It lets you focus on writing the implementations of your commands while it handles user
interaction, parsing etc.

Out of the box CMDR gives you;
- Command line parsing
- Command history
- Help functions and discoverability
- Auto completion (not yet implemented)

To use CMDR you write the commands you want your user to interact with as functions on one or
more Scope types. By implementing the scope trait cmdr can implement and execute your supplied
commands.
Implementing the Scope trait is as easy by using the supplied cmdr macro and annotating your
commands with the cmd annotation to provide useful metadata on your commands. Doc strings in
your command will be picked up automatically and used as help text.

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
### More information
- [API documentation](https://docs.rs/cmdr/)
- [Github repository](https://github.com/mendelt/cmdr)
- [Crates.io](https://crates.io/crates/cmdr)
- [Release notes](https://github.com/mendelt/cmdr/releases)

*version: 0.3.5*
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
