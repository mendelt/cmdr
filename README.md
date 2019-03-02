# cmdr &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://api.travis-ci.org/Mendelt/cmdr.svg?branch=master
[travis]: https://travis-ci.org/Mendelt/cmdr
[Latest Version]: https://img.shields.io/crates/v/cmdr.svg
[crates.io]: https://crates.io/crates/cmdr

**Cmdr is a library for building line-oriented text-based user interfaces in Rust.**

All you have to do is implement the functions you want a user to be able to execute as methods on a *Scope*
object. Add the cmdr macro annotation and let Cmdr handle the rest;
- Command line parsing
- Command history
- Auto completion (not yet implemented)
- Help functions and discoverability (not yet implemented)

Cargo.toml
```toml
[dependencies]
cmdr = "0.2"
```

main.rs
```rust
//! GreeterScope implements two commands, one greets the user with the supplied name. The other
//! returns CommandResult::Quit to quit the application.

use cmdr::*;

struct GreeterScope {}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone. Takes one parameter and prints a greeting
    fn do_greet(&self, args: &Vec<String>) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    fn do_quit(&self, _args: &Vec<String>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {});
}
```

## More information
- [API documentation](https://docs.rs/cmdr/)
- [Github repository](https://github.com/Mendelt/cmdr)
- [Crates.io](https://crates.io/crates/cmdr)
- [Release notes](https://github.com/Mendelt/cmdr/releases)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
