use cmdr::line_reader::{EchoLineReader, FileLineReader};
use cmdr::*;
use std::fs::File;

struct MainScope;

#[cmdr]
impl MainScope {
    #[cmd]
    fn command1(&mut self, args: &[String]) -> CommandResult {
        println!("command1 {}", args[0]);
        Ok(Action::Done)
    }

    #[cmd]
    fn sub(&mut self, _: &[String]) -> CommandResult {
        println!("sub");
        Action::sub_scope(SubScope {})
    }
}

struct SubScope;

#[cmdr]
impl SubScope {
    #[cmd]
    fn command2(&mut self, args: &[String]) -> CommandResult {
        println!("command2 {}", args[0]);
        Ok(Action::Done)
    }

    #[cmd]
    fn up(&mut self, _: &[String]) -> CommandResult {
        println!("up");
        Ok(Action::Exit)
    }
}

fn main() -> Result<()> {
    let line_reader = EchoLineReader::new(FileLineReader::new(
        File::open("./examples/09-file-input.txt").unwrap(),
    ));

    Runner::new(line_reader).run(&mut MainScope {})?;
    Ok(())
}
