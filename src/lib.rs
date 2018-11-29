use std::io::stdout;
use std::io::stdin;
use std::io::Write;


pub struct Loop { }


impl Loop {
    pub fn new() -> Loop {
        Loop { }
    }

    pub fn run(&self, context: &Context) {
        loop {
            print!("{} ", context.prompt());
            stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let command = input.trim();

            println!("{}", command);
        }
    }
}


pub trait Context {
    fn prompt(&self) -> String {
        ">".to_string()
    }
}
