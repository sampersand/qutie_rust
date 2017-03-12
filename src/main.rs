// #![allow(dead_code)]
#![allow(unused)]

macro_rules! get_option {
    ($e:expr) => (match $e {
        Some(e) => e,
        None => panic!(),
    });
    ($e:expr, $f:expr) => (match $e {
        Some(e) => e,
        None => panic!($f),
    });
}


mod logging;
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
   let text = "foo + 'a'";
// "
// a"
   let r = p.process(text);
   println!("{:?}", r.universe);
}



















