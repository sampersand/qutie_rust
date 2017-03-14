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
mod environment;
mod plugins;
mod parser;


fn main() {
   // what is access_t?
   println!("----");
   let mut p = parser::Parser::new();
   p.add_plugin(&plugins::number_plugin::INSTANCE);
   p.add_plugin(&plugins::whitespace_plugin::INSTANCE);
   p.add_plugin(&plugins::text_plugin::INSTANCE);
   p.add_plugin(&plugins::symbol_plugin::INSTANCE);
   // p.add_plugin(&plugins::operator_plugin::INSTANCE);
   let text = "12 + 1";
// "
// a"
   let r = p.process(text);
   println!("{:?}", r);
}



















