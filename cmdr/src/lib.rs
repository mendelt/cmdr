//! **Cmdr is a library for building line-oriented text-based user interfaces.**
//! It lets you focus on writing the implementations of your commands while it handles user
//! interaction, parsing etc.
//!
//! Out of the box CMDR gives you;
//! - Command line parsing
//! - Command history
//! - Help functions and discoverability
//! - Auto completion (not yet implemented)
//!
//! To use CMDR you write the commands you want your user to interact with as functions on one or
//! more Scope types. By implementing the scope trait cmdr can implement and execute your supplied
//! commands.
//! Implementing the Scope trait is as easy by using the supplied cmdr macro and annotating your
//! commands with the cmd annotation to provide useful metadata on your commands. Doc strings in
//! your command will be picked up automatically and used as help text.
//!
//! ```rust
//! use cmdr::*;
//! struct GreeterScope {}
//!
//! /// Example scope that implements two commands, greet and quit
//! #[cmdr]
//! impl GreeterScope {
//!     /// Cmdr command to greet someone. Takes one parameter and prints a greeting
//!     #[cmd]
//!     fn greet(&self, args: &[String]) -> CommandResult {
//!         println!("Hello {}", args[0]);
//!         CommandResult::Ok
//!     }
//!
//!     /// Cmdr command to quit the application by returning CommandResult::Quit
//!     #[cmd]
//!     fn quit(&self, _args: &[String]) -> CommandResult {
//!         println!("Quitting");
//!         CommandResult::Quit
//!     }
//! }
//!
//! /// Main function that creates the scope and starts a command loop for it
//! fn main() {
//!     cmd_loop(&mut GreeterScope {});
//! }
//! ```
//! ## More information
//! - [API documentation](https://docs.rs/cmdr/)
//! - [Github repository](https://github.com/mendelt/cmdr)
//! - [Crates.io](https://crates.io/crates/cmdr)
//! - [Release notes](https://github.com/mendelt/cmdr/releases)

// Turn on warnings for some lints
#[warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_qualifications
)]
mod line;
mod line_reader;
mod result;
mod scope;

use crate::line_reader::RustyLineReader;
pub use crate::line_reader::{LineReader, FileLineReader};
pub use crate::line::Line;
pub use crate::result::{CommandError, CommandResult};
pub use crate::scope::{Scope, ScopeCmdDescription, ScopeDescription};
pub use cmdr_macro::{cmd, cmdr};

/// This is the main entry-point to the cmdr library.
/// Creates a LineReader and executes its command on the scope that is passed to it.
pub fn cmd_loop<S: Scope>(scope: &mut S) -> CommandResult
{
    cmd_loop_from(scope, &mut RustyLineReader::new())
}

/// Use this as the entrypoint when using a different reader implementation.
pub fn cmd_loop_from<S: Scope, R: LineReader>(scope: &mut S, reader: &mut R) -> CommandResult {
    let mut result = scope.run_lines(reader);

    while let CommandResult::NewScope(scope_runner) = result {
        result = scope_runner.run_lines(reader);
    }

    result
}
