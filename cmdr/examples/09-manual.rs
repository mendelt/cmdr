//! Manual implementation of a cmdr application.
//! This example shows how to implement the Scope trait by hand. Normally you'd use the cmdr macro
//! to do the heavy lifting. But you can use cmdr without using macro's too.

use cmdr::*;

/// Example Cmdr scope
struct GreeterScope {}

impl GreeterScope {
    /// Cmdr command to greet someone.
    fn do_greet(&self, args: &[String]) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    /// Cmdr command to quit the application by returning CommandResult::Quit
    fn do_quit(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }

    fn commands(&mut self) -> CmdMethodList {
        CmdMethodList {
            methods: vec!(
                CmdMethod {
                    name: "greet".to_string(),
                    method: Box::new(|cmd_line| self.do_greet(&cmd_line.args))
                },
                CmdMethod {
                    name: "quit".to_string(),
                    method: Box::new(|cmd_line| self.do_quit(&cmd_line.args))
                },
            )
        }
    }
}

/// Manual scope implementation for Cmdr. Normally you'd use the cmdr macro for this. Implements
/// the command method that dispatches commands to functions implemented above.
impl Scope for GreeterScope {
    fn command(&mut self, command: &CommandLine) -> CommandResult {
        match self.commands().execute(command) {
            Some(result) => result,
            None => self.default(command)
        }

//        match &command.command[..] {
//            "greet" => self.do_greet(&command.args),
//            "quit" => self.do_quit(&command.args),
//            _ => self.default(&command),
//        }
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut GreeterScope {});
}

//
//struct CmdMethodList<S> {
//    methods: Vec<CmdMethod<S>>
//}
//
//
//impl<S> CmdMethodList<S> {
//    fn method_by_name(&self, name: &str) -> Option<CmdMethod<S>> {
//        self.methods.iter().filter(|method| method.name == name).next()
//    }
//
//    fn execute(&mut self, command: &CommandLine) -> CmdResult {
//        match self.method_by_name(&command.command) {
//            Some(method) => method.method(self, &command.args),
//            None => self.default(&command)
//        }
//    }
//}

//struct CmdMethod<S> {
//    name: String,
//    method: fn(&mut S, CommandLine) -> CommandResult
//}

struct CmdMethodList {
    methods: Vec<CmdMethod>
}


impl CmdMethodList {
    fn method_by_name(&self, name: &str) -> Option<&CmdMethod> {
        self.methods.iter().filter(|method| method.name == name).next()
    }

    fn execute(&mut self, command: &CommandLine) -> Option<CommandResult> {
        match self.method_by_name(&command.command) {
            Some(method) => Some((method.method)(&command)),
            None => None
        }
//        match self.method_by_name(&command.command) {
//            Some(method) => method.method(self, &command.args),
//        }
    }
}

struct CmdMethod {
    name: String,
    method: Box<Fn(&CommandLine) -> CommandResult>,
}