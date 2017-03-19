#![allow(unused)]
#[macro_use]
extern crate lazy_static;

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
what is access_t?
- oper.handle_rhs should use env.fork and maybe a new function called env.rebase
- determine what to do about _eql -> either have everythign like _bool and _text for speed, or use none
*/

// use std::io;
// use std::io::File;
// use std::path::Path;
// fn lines_from_file<P>(filename: P) -> Result<io::BufReader<File>, io::Error>
//    where P: AsRef<Path> {
//    let mut file = try!(File::open(filename));
//    Ok(io::BufReader::new(file)))
// }
fn main() {
   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new();
   p.add_plugin(&plugins::number_plugin::INSTANCE);
   p.add_plugin(&plugins::whitespace_plugin::INSTANCE);
   p.add_plugin(&plugins::comment_plugin::INSTANCE);
   p.add_plugin(&plugins::text_plugin::INSTANCE);
   p.add_plugin(&plugins::symbol_plugin::INSTANCE);
   p.add_plugin(&plugins::operator_plugin::INSTANCE);
   p.add_plugin(&plugins::universe_plugin::INSTANCE);
   let text = "
my_array = [1, 2, 3]!;
my_dict = {
   a = 1;
   b = 2;
   3 = c;
}!;
my_dict?.3
";
   let r = p.process(text);
   println!("====[ Results ]====");
   println!("{}", r);
}




















//    let text = "
// Car = {
//   maker  = maker?  | 'honda';
//   wheels = wheels? | 4;

//   __text = {
//     'I\'m a ' + __self?.maker + ' with ' + __self?.wheels + ' wheels!'r
//   };

//   drive = { 
//     -||> dist;
//     disp( if(dist?, 'I drove ' + dist? + ' miles!', 'vroom vroom!'));
//   };

// };

// car = new Car(maker: toyoya);
// disp( text(car?) );
// car?.drive( 5 );
// car?.drive@(( dist: 9.3 )@()!)!.0;
// car?.drive();
// ";
