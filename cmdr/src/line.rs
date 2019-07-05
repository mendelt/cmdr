use crate::CommandError;

/// A parsed line from the user
#[derive(Debug, PartialEq)]
pub struct Line {
    /// The command name
    pub command: String,

    /// A vector of strings containing the arguments of the command
    pub args: Vec<String>,
}

impl Line {
    /// Try to parse a Line from a String or return an error when unsuccesfull
    pub fn try_parse(line: &str) -> Result<Line, CommandError> {
        let mut parts = line.trim().split(' ').filter(|part| !part.is_empty());

        let first = parts.next();

        match first {
            None => Err(CommandError::EmptyLine),
            Some(command) => Ok(Line {
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
        let line = Line::try_parse("");

        assert_eq!(line, Err(CommandError::EmptyLine));
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command() {
        let line = Line::try_parse("command");

        assert_eq!(
            line,
            Ok(Line {
                command: "command".to_string(),
                args: [].to_vec()
            })
        );
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command_with_arguments() {
        let line = Line::try_parse("command with arguments");

        assert_eq!(
            line,
            Ok(Line {
                command: "command".to_string(),
                args: ["with".to_string(), "arguments".to_string()].to_vec()
            })
        );
    }
}
