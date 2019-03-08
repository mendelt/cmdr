use crate::line_reader::LineReader;
use crate::CommandLine;
use crate::Line;

/// A command result. returned by one of the client-implemented command methods
#[derive(Debug, PartialEq)]
pub enum CommandResult {
    /// Result Ok, ready to go on to the next command
    Ok,

    /// Result Quit, close the application and stop
    Quit,
}

/// Trait for implementing a Scope object. This trait can be implemented by a client but will most
/// likely be implemented for you by the cmdr macro.
pub trait Scope {
    /// Execute commands in this scope. Uses a LineReader to get commands and executes them one by
    /// one until a command returns CommandResult::Quit
    fn run_lines(&mut self, reader: &mut LineReader) -> CommandResult {
        self.before_loop();

        let mut last_result = CommandResult::Ok;

        while last_result == CommandResult::Ok {
            last_result = self.run_line(reader.read_line(self.prompt().as_ref()));
        }

        self.after_loop();
        last_result
    }

    /// Execute a single line
    fn run_line(&mut self, line: Line) -> CommandResult {
        let line = self.before_command(line);
        let result = match line {
            Line::Empty => self.empty(),
            Line::CtrlC | Line::CtrlD | Line::Error => CommandResult::Quit,
            Line::Command(ref command) => self.command(command),
        };

        self.after_command(&line, result)
    }

    /// Execute a single command, must be implemented by trait implementors or by the cmdr macro
    fn command(&mut self, command: &CommandLine) -> CommandResult;

    /// Return the prompt for this scope. The default implementation returns > as the prompt but
    /// this can be overridden to return other strings or implement dynamically generated prompts
    fn prompt(&self) -> String {
        ">".to_string()
    }

    /// Execute an empty line.
    /// The default implentation does nothing but this can be overridden by a client-application
    /// to implement other behaviour
    fn empty(&mut self) -> CommandResult {
        CommandResult::Ok
    }

    /// A user entered an unknown command.
    /// The default implementation prints an error to the user and returns ok to go on. Can be
    /// overridden by a client-application to implement other behaviour
    fn default(&mut self, _command: &CommandLine) -> CommandResult {
        println!("Unknown command");
        CommandResult::Ok
    }

    /// Hook that is called before the command loop starts, can be overridden
    fn before_loop(&mut self) {}

    /// Hook that is called before executing a command, can be overridden
    fn before_command(&mut self, line: Line) -> Line {
        line
    }

    /// Hook that is called after command execution is finished, can be overridden
    fn after_command(&mut self, _line: &Line, result: CommandResult) -> CommandResult {
        result
    }

    /// Hook that is called after the command loop finishes, can be overridden
    fn after_loop(&mut self) {}
}

/// List of command methods implemented by a scope
pub struct CmdMethodList<T> {
    methods: Vec<CmdMethod<T>>,
}

impl<T> CmdMethodList<T> {
    /// Construct a command method list
    pub fn new(methods: Vec<CmdMethod<T>>) -> Self {
        CmdMethodList { methods }
    }

    /// Find a command method by it's command name
    pub fn method_by_command(&self, name: &str) -> Option<&CmdMethod<T>> {
        self.methods
            .iter()
            .filter(|method| method.name == name)
            .next()
    }
}

/// All information about a command method in one handy struct
pub struct CmdMethod<T> {
    name: String,
    method: Box<Fn(&mut T, &CommandLine) -> CommandResult>,
}

impl<T> CmdMethod<T> {
    /// Construct a CmdMethod from a command name and a command closure
    pub fn new(name: String, method: Box<Fn(&mut T, &CommandLine) -> CommandResult>) -> Self {
        CmdMethod { name, method }
    }

    /// Execute this command
    pub fn execute(&self, scope: &mut T, command: &CommandLine) -> CommandResult {
        (self.method)(scope, command)
    }
}
