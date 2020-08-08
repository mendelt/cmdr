use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::{result::Action, CommandResult};
use std::fmt::Debug;

/// Wraps a LineReader and a Scope and allows using the scope to interpret commands from the
/// LineReader
#[derive(Debug)]
pub struct Runner<R: LineReader> {
    reader: R,
}

impl<R: LineReader> Runner<R> {
    /// Create a new runner that takes lines from the `reader` and executes them using the `scope`
    pub fn new(reader: R) -> Self {
        Runner { reader: reader }
    }

    /// Start reading lines and executing them
    pub fn run<S: Scope + Sized>(&mut self, scope: &mut S) -> CommandResult {
        let mut result = scope.run_lines(&mut self.reader);

        while let CommandResult::Ok(Action::NewScope(scope_runner)) = result {
            result = scope_runner.run_lines(&mut self.reader);
        }

        result
    }
}
