use cmdr::{cmd_loop, Context, Line};
use cmdr::CommandResult;


struct MyContext { }


impl MyContext {
    pub fn quit(&self, args: Vec<&str>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}


impl Context for MyContext {
    fn prompt(&self) -> String {
        "#".to_string()
    }

    fn command(&mut self, line: Line) -> CommandResult {
        match line {
            Line::Empty => self.empty(),
            Line::Command("quit", args) => self.quit(args),
            _ => self.default(line)
        }
    }
}


fn main(){
    let mut context = MyContext {};
    cmd_loop(&mut context);
}
