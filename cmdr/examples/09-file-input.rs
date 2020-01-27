use cmdr::line_reader::EchoLineReader;
use cmdr::*;
use std::fs::File;

struct MainScope;

#[cmdr]
impl MainScope {
    #[cmd]
    fn command1(&mut self, args: &[String]) -> CommandResult {
        println!("command1 {}", args[0]);
        CommandResult::Ok
    }

    #[cmd]
    fn sub(&mut self, _: &[String]) -> CommandResult {
        println!("sub");
        CommandResult::sub_scope(SubScope {})
    }
}

struct SubScope;

#[cmdr]
impl SubScope {
    #[cmd]
    fn command2(&mut self, args: &[String]) -> CommandResult {
        println!("command2 {}", args[0]);
        CommandResult::Ok
    }

    #[cmd]
    fn up(&mut self, _: &[String]) -> CommandResult {
        println!("up");
        CommandResult::Exit
    }
}

fn main() {
    let mut line_reader = EchoLineReader::new(FileLineReader::new(
        File::open("./examples/09-file-input.txt").unwrap(),
    ));
    // TODO: This syntax will probably change before version 0.3.12
    cmd_loop_from(&mut MainScope {}, &mut line_reader);
}
