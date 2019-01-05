/// A parsed line from the user
#[derive(Debug, PartialEq)]
pub enum Line<'a> {
    /// An empty line
    Empty,

    /// A user command made up of a command and a series of attributes
    Command(&'a str, Vec<&'a str>),
}

pub fn parse_line(line: &str) -> Line {
    let mut parts = line.trim().split(' ').filter(|part| !part.is_empty());

    match parts.next() {
        None => Line::Empty,
        Some(command) => Line::Command(command, parts.collect()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test parsing an empty line
    fn test_parse_empty_line() {
        assert_eq!(parse_line(""), Line::Empty);
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command() {
        assert_eq!(parse_line("command"), Line::Command("command", [].to_vec()));
    }

    #[test]
    // Test parsing an empty line
    fn test_parse_command_with_arguments() {
        assert_eq!(
            parse_line("command with arguments"),
            Line::Command("command", ["with", "arguments"].to_vec())
        );
    }

}
