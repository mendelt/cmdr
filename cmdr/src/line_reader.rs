use crate::CommandError;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{BufRead, BufReader, Read};

/// Linereader trait, a line reader gets lines from a user, for example from the command line and
/// parses them.
pub trait LineReader {
    fn read_line(&mut self, prompt: &str) -> Result<String, CommandError>;
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
    fn read_line(&mut self, prompt: &str) -> Result<String, CommandError> {
        let input = self.editor.readline(format!("{} ", prompt).as_ref());
        match input {
            Ok(line_string) => {
                let string_ref: &str = line_string.as_ref();
                self.editor.add_history_entry(string_ref);
                Ok(line_string)
            }
            Err(ReadlineError::Interrupted) => Err(CommandError::CtrlC),
            Err(ReadlineError::Eof) => Err(CommandError::CtrlD),
            Err(_) => Err(CommandError::LineReaderError),
        }
    }
}

/// Wraps a LineReader and echoes all read lines
pub struct EchoLineReader<W: LineReader> {
    wrapped: W,
}

impl<W: LineReader> EchoLineReader<W> {
    pub fn new(wrapped: W) -> Self {
        EchoLineReader { wrapped }
    }
}

impl<W: LineReader> LineReader for EchoLineReader<W> {
    fn read_line(&mut self, prompt: &str) -> Result<String, CommandError> {
        match self.wrapped.read_line(prompt) {
            Ok(line) => {
                println!("{} {}", prompt, &line);
                Ok(line)
            }
            Err(error) => Err(error),
        }
    }
}

/// Read commands from an io stream like a textfile or domain socket
pub struct FileLineReader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> FileLineReader<R> {
    /// Create a new FileLineReader
    pub fn new(reader: R) -> Self {
        FileLineReader {
            reader: BufReader::new(reader),
        }
    }
}

impl<R: Read> LineReader for FileLineReader<R> {
    fn read_line(&mut self, _: &str) -> Result<String, CommandError> {
        let mut line = String::new();
        let input = self.reader.read_line(&mut line);
        match input {
            Ok(0) => Err(CommandError::CtrlD),
            Ok(_) => Ok(line),
            Err(_) => Err(CommandError::LineReaderError),
        }
    }
}
