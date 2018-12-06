use cmdr::{ContextMacro};

#[derive(ContextMacro)]
struct MyContext { }


fn main(){
    let context = MyContext {};
    context.do_stuff();
}
