use crate::line::*;
use crate::CommandError;
use rustyline::error::ReadlineError;
use rustyline::Editor;

/// Linereader trait, a line reader gets lines from a user, for example from the command line and
/// parses them.
pub trait LineReader {
    fn read_line(&mut self, prompt: &str) -> Result<Line, CommandError>;
}

/// Implementation of the LineReader trait using the rustyline library
pub struct RustyLineReader {
    editor: Editor<()>,
}

impl RustyLineReader {
    pub fn new() -> Self {
        RustyLineReader {
            editor: Editor::<()>::new(),
        }
    }
}

impl LineReader for RustyLineReader {
    fn read_line(&mut self, prompt: &str) -> Result<Line, CommandError> {
        let input = self.editor.readline(format!("{} ", prompt).as_ref());
        match input {
            Ok(line_string) => {
                let string_ref: &str = line_string.as_ref();
                self.editor.add_history_entry(string_ref);
                Line::try_parse(string_ref.into())
            }
            Err(ReadlineError::Interrupted) => Err(CommandError::CtrlC),
            Err(ReadlineError::Eof) => Err(CommandError::CtrlD),
            Err(_) => Err(CommandError::LineReaderError),
        }
    }
}
