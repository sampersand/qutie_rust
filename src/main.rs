// #![allow(dead_code)]
#![allow(unused)]

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
   p.add_plugin(&plugins::operator_plugin::INSTANCE);
   // let text = "foo + 'a' * 194.3";
   let text = "a = 1";
// "
// a"
   let r = p.process(text);
   println!("{:?}", r.universe);
}



















