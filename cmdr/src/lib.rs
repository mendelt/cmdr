//! **Cmdr is a library for building line-oriented text-based user interfaces.**
//!
//! This can be done by implementing one or more objects that implement the Cmdr::Scope trait. A
//! command loop can then be started on a scope by calling the cmd_loop function. The command loop
//! uses a line reader to get user input and executes them by running the appropriate functions on
//! the supplied scope object.
//!
//! Implementing a scope is as easy as creating an object with a few methods that take a vector of
//! &str as their input and return a CommandResult. By annotating the impl block of that object
//! the cmdr macro all functions starting annotated with #[cmd] in that block can be used as
//! commands.
//!
//! For additional functionality like setting custom prompts or setting hooks to catch unknown or
//! empty commands additional methods can be added to the impl block. These correspond to
//! overridable functions in the Scope trait.

mod line;
mod line_reader;
mod scope;

use crate::line_reader::RustyLineReader;

pub use crate::line::{CommandLine, Line};
pub use crate::scope::{CommandResult, Scope, ScopeCmdDescription, ScopeDescription};
pub use cmdr_macro::{cmd, cmdr};

/// This is the main entry-point to the cmdr library.
/// Creates a LineReader and executes its command on the scope that is passed to it.
pub fn cmd_loop<T>(scope: &mut T)
where
    T: Scope,
{
    let mut reader = RustyLineReader::new();
    scope.run_lines(&mut reader);
}
