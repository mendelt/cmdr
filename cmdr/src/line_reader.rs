use crate::line::*;
use crate::CommandError;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{BufRead, BufReader, Read};

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

pub struct FileLineReader<R: Read> {
    reader: BufReader<R>,
    echo: bool,
}

impl<R: Read> FileLineReader<R> {
    pub fn new(reader: R) -> Self {
        FileLineReader {
            reader: BufReader::new(reader),
            echo: false,
        }
    }

    pub fn echo_on(mut self) -> Self {
        self.echo = true;
        self
    }
}

impl<R: Read> LineReader for FileLineReader<R> {
    fn read_line(&mut self, prompt: &str) -> Result<Line, CommandError> {
        let mut line = String::new();
        let input = self.reader.read_line(&mut line);
        match input {
            Ok(0) => Err(CommandError::CtrlD),
            Ok(_) => {
                if self.echo {
                    println!("{} {}", prompt, &line);
                }
                Line::try_parse(line.as_ref())
            }
            Err(_) => Err(CommandError::LineReaderError),
        }
    }
}
