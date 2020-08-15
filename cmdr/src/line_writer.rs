//! Contains the LineWriter trait and some implementations to write lines to several destinations
use std::io::{stdout, Write};

/// A line writer handles output by writing lines of output to its destination
pub trait LineWriter {
    /// Write a line, add a carriage return
    fn write_line(&mut self, line: &str) {
        self.write(line);
        self.write("\n");
    }

    /// Write a string
    fn write(&mut self, line: &str);
}

/// Write lines to stdout
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PrintlnWriter {}

impl LineWriter for PrintlnWriter {
    fn write(&mut self, line: &str) {
        stdout().write(line.as_bytes()).ok();
    }
}
