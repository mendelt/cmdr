use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::{line_writer::LineWriter, result::Action, CommandResult};
use std::fmt::Debug;

/// Wraps a LineReader and a Scope and allows using the scope to interpret commands from the
/// LineReader
#[derive(Debug)]
pub struct Runner<R: LineReader, W: LineWriter> {
    reader: R,
    writer: W,
}

impl<R: LineReader, W: LineWriter> Runner<R, W> {
    /// Create a new runner that takes lines from the `reader` and executes them using the `scope`
    pub fn new(reader: R, writer: W) -> Self {
        Runner { reader, writer }
    }

    /// Start reading lines and executing them
    pub fn run<S: Scope>(&mut self, scope: &mut S) -> CommandResult {
        let mut result = scope.run_lines(&mut self.reader, &mut self.writer);

        while let CommandResult::Ok(Action::NewScope(scope_runner)) = result {
            result = scope_runner.run_lines(&mut self.reader, &mut self.writer);
        }

        result
    }
}
