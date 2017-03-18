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


fn main() {
   // what is access_t?
   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new();
   p.add_plugin(&plugins::number_plugin::INSTANCE);
   p.add_plugin(&plugins::whitespace_plugin::INSTANCE);
   p.add_plugin(&plugins::text_plugin::INSTANCE);
   p.add_plugin(&plugins::symbol_plugin::INSTANCE);
   p.add_plugin(&plugins::operator_plugin::INSTANCE);
   let text = "
x = 1;
y = 2;
x?
";
   let r = p.process(text);
   println!("====[ Results ]====");
   println!("{}", r);
}



















