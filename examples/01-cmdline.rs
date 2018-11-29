use cmdr::{Loop, Context};

struct MyContext { }


impl Context for MyContext {
    fn prompt(&self) -> String {
        "#".to_string()
    }
}


fn main(){
    let context = MyContext {};
    Loop::new().run(&context);
}
