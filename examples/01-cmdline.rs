use cmdr::{cmd_loop, Context, Line};


struct MyContext { }


impl MyContext {
    pub fn quit(&self, args: Vec<&str>) {
        println!("quitterdequit!");
    }
}


impl Context for MyContext {
    fn prompt(&self) -> String {
        "#".to_string()
    }

    fn command(&mut self, line: Line) {
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
