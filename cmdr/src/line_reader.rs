use crate::line::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;

pub trait LineReader {
    fn read_line(&mut self, prompt: &str) -> Line;
}

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
                self.editor.add_history_entry(line_string.as_ref());
                line_string[..].into()
            }
            Err(ReadlineError::Interrupted) => Line::CtrlC,
            Err(ReadlineError::Eof) => Line::CtrlD,
            Err(_) => Line::Error,
        }
    }
}
