#![allow(unused)]
#[macro_use]
extern crate lazy_static;

macro_rules! unwrap {
    ( $x:expr, $y:expr ) => {
        {
            match $x {
               Some(e) => e,
               None => panic!($y)
            }
        }
    };
}


mod objects;
mod plugins;
mod parser;
mod result;
mod env;

mod globals {
   use env::Environment;
   use std::rc::Rc;

   pub static mut GLOBAL_ENV: *mut Environment<'static> = 0 as *mut Environment<'static>;

}
/*
TODO
- oper.handle_rhs should use env.fork and maybe a new function called env.rebase
- determine what to do about _eql -> either have everythign like _bool and _text for speed, or use none
*/

fn main() {
   // what is access_t?
   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new();
   p.add_plugin(&plugins::number_plugin::INSTANCE);
   p.add_plugin(&plugins::whitespace_plugin::INSTANCE);
   p.add_plugin(&plugins::text_plugin::INSTANCE);
   p.add_plugin(&plugins::symbol_plugin::INSTANCE);
   p.add_plugin(&plugins::operator_plugin::INSTANCE);
   p.add_plugin(&plugins::universe_plugin::INSTANCE);
   let text = "
add_xy = {
   x? + y?
};
add_xy? @ (x = 3, y = 4)!,.0;

";
   let r = p.process(text);
   println!("====[ Results ]====");
   println!("{}", r);
}



















