use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::collections::HashMap;


pub fn cmd_loop(context: &Context) {
    loop {
        print!("{} ", context.prompt());
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let line = input.trim();

        one_line(line.split(" ").collect());
    }
}


pub fn one_line(line: Vec<&str>) {
    println!("{}", line[0]);
}


pub trait Context {
    fn prompt(&self) -> String {
        ">".to_string()
    }
}
