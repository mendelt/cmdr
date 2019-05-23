use crate::line::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

/// Linereader trait, a line reader gets lines from a user, for example from the command line and
/// parses them.
pub trait LineReader {
    fn read_line(&mut self, prompt: &str) -> Line;
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
    fn read_line(&mut self, prompt: &str) -> Line {
        let input = self.editor.readline(format!("{} ", prompt).as_ref());
        match input {
            Ok(line_string) => {
                let string_ref: &str = line_string.as_ref();
                self.editor.add_history_entry(string_ref);
                string_ref.into()
            }
            Err(ReadlineError::Interrupted) => Line::CtrlC,
            Err(ReadlineError::Eof) => Line::CtrlD,
            Err(_) => Line::Error,
        }
    }
}
