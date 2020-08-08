//! Contains the LineReader trait and several implementations to read lines from several sources

use crate::Error;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{BufRead, BufReader, Read};

/// Linereader trait, a line reader gets lines from a user, for example from the command line and
/// parses them.
pub trait LineReader {
    /// Blocks until a new line is entered
    fn read_line(&mut self, prompt: &str) -> Result<String, Error>;
}

/// Implementation of the LineReader trait using the rustyline library
#[derive(Debug)]
pub struct RustyLineReader {
    editor: Editor<()>,
}

impl RustyLineReader {
    /// Construct and return an new `RustyLineReader`
    pub fn new() -> Self {
        RustyLineReader {
            editor: Editor::<()>::new(),
        }
    }
}

impl LineReader for RustyLineReader {
    fn read_line(&mut self, prompt: &str) -> Result<String, Error> {
        let input = self.editor.readline(format!("{} ", prompt).as_ref());
        match input {
            Ok(line_string) => {
                let string_ref: &str = line_string.as_ref();
                self.editor.add_history_entry(string_ref);
                Ok(line_string)
            }
            Err(ReadlineError::Interrupted) => Err(Error::CtrlC),
            Err(ReadlineError::Eof) => Err(Error::CtrlD),
            Err(_) => Err(Error::LineReaderError),
        }
    }
}

/// Wraps a LineReader and echoes all read lines
#[derive(Debug)]
pub struct EchoLineReader<W: LineReader> {
    wrapped: W,
}

impl<W: LineReader> EchoLineReader<W> {
    /// Construct and return an new `EchoLineReader`
    pub fn new(wrapped: W) -> Self {
        EchoLineReader { wrapped }
    }
}

impl<W: LineReader> LineReader for EchoLineReader<W> {
    fn read_line(&mut self, prompt: &str) -> Result<String, Error> {
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
#[derive(Debug)]
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
    fn read_line(&mut self, _: &str) -> Result<String, Error> {
        let mut line = String::new();
        let input = self.reader.read_line(&mut line);
        match input {
            Ok(0) => Err(Error::CtrlD),
            Ok(_) => Ok(line),
            Err(_) => Err(Error::LineReaderError),
        }
    }
}
