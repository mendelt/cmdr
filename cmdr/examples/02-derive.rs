use cmdr::*;

struct ExampleScope { }

#[cmdr]
impl ExampleScope {
    fn do_testcommand1(&self, args: Vec<&str>) -> CommandResult {
        println!("command 1");
        CommandResult::Succes
    }

    fn do_testcommand2(&self, args: Vec<&str>) -> CommandResult {
        println!("command 2");
        CommandResult::Succes
    }

    fn do_quit(&self, args: Vec<&str>) -> CommandResult {
        CommandResult::Quit
    }
}


fn main(){
    let mut scope = ExampleScope {};
    cmd_loop(&mut scope);
}
