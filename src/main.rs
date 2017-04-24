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
mod test;

mod globals {
   pub type IdType = u32;
   pub static mut CURRENT_ID: IdType = 0;
}

mod execute {
   use std::fs::File;
   use std::io::Read;
   use parser::Parser;
   use objects::universe::Universe;
   pub fn read_file(path: &str) -> String {
      let mut text = String::new();
      match File::open(path) {
         Ok(mut file) => 
            if let Err(err) = file.read_to_string(&mut text){
               panic!("Cannot read file {:?} to string: {:?}", path, err)
            },
         Err(err) => panic!("Cannot open file {:?} for reading: {:?}", path, err)
      };
      text
   }

   pub fn run(text: &str) -> Universe {
      Parser::new().process(text)
   }
}



fn main() {
   let path = "/Users/westerhack/code/rust/qutie_rust/examples/example.qt";
   let text = execute::read_file(path);
   println!("====[ Runtime ]====");
   let ret = execute::run(&text);
   println!("====[ Results ]====");
   println!("{}", ret);
}









