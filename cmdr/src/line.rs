use std::fmt::Display;

/// A parsed line from the user
#[derive(Debug, PartialEq)]
pub enum Line<'a> {
    /// An empty line
    Empty,

    /// A user command made up of a command and a series of attributes
    Command(CommandLine<'a>),
}

/// A parsed command, optionally with arguments
#[derive(Debug, PartialEq)]
pub struct CommandLine<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(line: &'a str) -> Self {
        let mut parts = line.trim().split(' ').filter(|part| !part.is_empty());

        match parts.next() {
            None => Line::Empty,
            Some(command) => Line::Command(CommandLine {
                command,
                args: parts.collect(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test parsing an empty line
    fn test_parse_empty_line() {
        let line: Line = "".into();

        assert_eq!(line, Line::Empty);
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command() {
        let line: Line = "command".into();

        assert_eq!(
            line,
            Line::Command(CommandLine {
                command: "command",
                args: [].to_vec()
            })
        );
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command_with_arguments() {
        let line: Line = "command with arguments".into();

        assert_eq!(
            line,
            Line::Command(CommandLine {
                command: "command",
                args: ["with", "arguments"].to_vec()
            })
        );
    }

}
