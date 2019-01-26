/// A parsed line from the user
#[derive(Debug, PartialEq)]
pub enum Line {
    /// An empty line
    Empty,

    /// A user command made up of a command and a series of attributes
    Command(CommandLine),
}

/// A parsed command, optionally with arguments
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>,
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let mut parts = line.trim().split(' ').filter(|part| !part.is_empty());

        let first = parts.next();

        match first {
            None => Line::Empty,
            Some(command) => Line::Command(CommandLine {
                command: command.to_string(),
                args: parts.map(|arg| arg.to_string()).collect(),
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
                command: "command".to_string(),
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
                command: "command".to_string(),
                args: ["with".to_string(), "arguments".to_string()].to_vec()
            })
        );
    }

}
