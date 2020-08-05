use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::CommandResult;

/// Wraps a LineReader and a Scope and allows using the scope to interpret commands from the
/// LineReader
pub struct Runner<S: Scope> {
    reader: Box<dyn LineReader>,
    scope: S,
}

impl<S: Scope> Runner<S> {
    /// Create a new runner that takes lines from the `reader` and executes them using the `scope`
    pub fn new(reader: Box<dyn LineReader>, scope: S) -> Self {
        Runner { reader, scope }
    }

    /// Start reading lines and executing them
    pub fn run(&mut self) -> CommandResult {
        let mut result = self.scope.run_lines(self.reader.as_mut());

        while let CommandResult::NewScope(scope_runner) = result {
            result = scope_runner.run_lines(self.reader.as_mut());
        }

        result
    }
}
