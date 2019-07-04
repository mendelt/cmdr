use crate::CommandError;

/// A parsed line from the user
#[derive(Debug, PartialEq)]
pub enum Line {
    /// A user command made up of a command and a series of attributes
    Command(CommandLine),

    /// A user help request
    Help(CommandLine),
}

/// A parsed command, optionally with arguments
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    /// The command name
    pub command: String,

    /// A vector of strings containing the arguments of the command
    pub args: Vec<String>,
}

impl Line {
    fn try_parse(line: &str) -> Result<Line, CommandError> {
        let mut parts = line.trim().split(' ').filter(|part| !part.is_empty());

        let first = parts.next();

        match first {
            None => Err(CommandError::EmptyLine),
            Some("help") => Ok(Line::Help(CommandLine {
                command: "help".to_string(),
                args: parts.map(|arg| arg.to_string()).collect(),
            })),
            Some(command) => Ok(Line::Command(CommandLine {
                command: command.to_string(),
                args: parts.map(|arg| arg.to_string()).collect(),
            })),
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
            Ok(Line::Command(CommandLine {
                command: "command".to_string(),
                args: [].to_vec()
            }))
        );
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command_with_arguments() {
        let line= Line::try_parse("command with arguments");

        assert_eq!(
            line,
            Ok(Line::Command(CommandLine {
                command: "command".to_string(),
                args: ["with".to_string(), "arguments".to_string()].to_vec()
            }))
        );
    }
}
