use cmdr::{cmd_loop, Context};


struct MyContext { }


impl Context for MyContext {
    fn prompt(&self) -> String {
        "#".to_string()
    }
}


fn main(){
    let context = MyContext {};
    cmd_loop(&context);
}
