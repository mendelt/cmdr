# cmdr &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://api.travis-ci.org/Mendelt/cmdr.svg?branch=master
[travis]: https://travis-ci.org/Mendelt/cmdr
[Latest Version]: https://img.shields.io/crates/v/cmdr.svg
[crates.io]: https://crates.io/crates/cmdr

**Cmdr is a library for building line-oriented text-based user interfaces in Rust.**

All you have to do is implement the functions you want a user to be able to execute as methods on a *Scope*
object. Add the cmdr macro annotation and let Cmdr handle the rest;
- Command line parsing
- Command history (not implemented)
- Auto completion (not implemented)
- Help functions and discoverability (not implemented)

Cargo.toml
```toml
[dependencies]
cmdr = "0.1"
```

main.rs
```rust
use cmdr::*;

struct GreeterScope { }

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl GreeterScope {
    /// Cmdr command to greet someone. Takes one parameter and prints a greeting
    pub fn do_greet(&self, args: Vec<&str>) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    pub fn do_quit(&self, _args: Vec<&str>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main(){
    let mut scope = GreeterScope {};
    cmd_loop(&mut scope);
}
```

## More information
- [API documentation](https://docs.rs/cmdr/)
- [Github repository](https://github.com/Mendelt/cmdr)
- [Crates.io](https://crates.io/crates/cmdr)
- [Release notes](https://github.com/Mendelt/cmdr/releases)

## License
Cmdr is licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
