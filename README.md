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
```
[dependencies]
cmdr = "0.1"
```

main.rs
```
use cmdr::*;

struct ExampleScope { }

#[cmdr]
impl ExampleScope {
    fn do_testcommand1(&self, args: Vec<&str>) -> CommandResult {
        println!("command 1");
        CommandResult::Succes
    }

    fn do_testcommand2(&self, args: Vec<&str>) -> CommandResult {
        println!("command 2");
        CommandResult::Succes
    }

    fn do_quit(&self, args: Vec<&str>) -> CommandResult {
        CommandResult::Quit
    }
}


fn main(){
    let mut scope = ExampleScope {};
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
