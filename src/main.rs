#![allow(dead_code)]
#![allow(unused)]

mod util;
mod objects;
mod environment;
mod plugins;
mod parser;


fn main() {
   println!("----");
   let p = parser::Parser::new();
   // let r = p.process("1 + 2");
   let r = p.process("1 + 2");
}
