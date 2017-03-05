#![allow(dead_code)]
#![allow(unused)]

mod objects;
mod environment;
mod plugin;
mod parser;

fn main() {
   let p = parser::Parser::new();
   // let r = p.process("1 + 2");
   let r = p.process("1 + 2");
   let ref value = *r.stream.stack[0];
   println!("{:?}", r);
}
