use crate::line_reader::LineReader;
use crate::scope::Scope;
use crate::CommandResult;

pub struct Runner<S: Scope> {
    reader: Box<dyn LineReader>,
    scope: S,
}

impl<S: Scope> Runner<S> {
    pub fn new(reader: Box<dyn LineReader>, scope: S) -> Self {
        Runner { reader, scope }
    }

    pub fn run(&mut self) -> CommandResult {
        let mut result = self.scope.run_lines(self.reader.as_mut());

        while let CommandResult::NewScope(scope_runner) = result {
            result = scope_runner.run_lines(self.reader.as_mut());
        }

        result
    }
}
