#![allow(unused)]
#[macro_use]
extern crate guard;

#[macro_use]
extern crate lazy_static;

extern crate regex;

#[macro_use]
mod qt_macros;
#[macro_use]
mod debugging;

mod objects;
mod plugins;
mod parser;
mod result;
mod builtins;
mod env;
mod stream;


mod globals {
   use env::Environment;
   pub type IdType = u32;
   pub static mut CURRENT_ID: IdType = 0;
}
/*
TODO
- oper.handle_rhs should use env.fork and maybe a new function called env.rebase
- determine what to do about _eql -> either have everythign like _bool and _text for speed, or use none
*/

fn main() {
   let inp_file = "/Users/westerhack/code/rust/qutie_rust/examples/example.qt";

   use std::fs::File;
   use std::io::Read;

   let mut text = String::new();

   match File::open(inp_file) {
      Ok(file) => file,
      Err(err) => panic!("Cannot open file {:?} for reading: {}", inp_file, err)
   }.read_to_string(&mut text);

   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new();

   let r = p.process(text.as_str());
   println!("====[ Results ]====");
   println!("{}", r);
   
}


