use cmdr::*;


struct ExampleScope { }


impl ExampleScope {
    pub fn do_greet(&self, args: Vec<&str>) -> CommandResult {
        println!("Hello {}", args[0]);
        CommandResult::Succes
    }

    pub fn do_quit(&self, args: Vec<&str>) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}


impl Scope for ExampleScope {
    fn command(&mut self, line: Line) -> CommandResult {
        match line {
            Line::Empty => self.empty(),
            Line::Command("greet", args) => self.do_greet(args),
            Line::Command("quit", args) => self.do_quit(args),
            _ => self.default(line)
        }
    }
}


fn main(){
    let mut context = ExampleScope {};
    cmd_loop(&mut context);
}
