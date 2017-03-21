#![allow(unused)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

#[macro_use]
mod qt_macros {
   macro_rules! obj_functions {
      (QT_TO_BOOL; $bool_expr:expr) => {
         fn qt_to_bool(&self, _: &mut Environment) -> BoolResult {
            let ans = ($bool_expr)(self); /* is a closure, for now. Later on i'll figure out how to fix that */
            ok_rc!(Boolean::from_bool(ans))
         }
      };
      (QT_TO_TEXT) => {
         fn qt_to_text(&self, _: &mut Environment) -> Result<Rc<Text>, ObjError> {
            use objects::text::Quote;
            ok_rc!(Text::new(self.to_string(), [Quote::Single, Quote::Single]))
         }
      };
      (QT_EQL; $obj_type:ident, $comp_item:ident) => {
         fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> BoolResult {
            let other = match other.obj_type() {
               ObjType::$obj_type(ele) => ele,
               _ => return Err(ObjError::NotImplemented)
            };
            ok_rc!(Boolean::from_bool(self.$comp_item == other.$comp_item))
         }
         fn qt_eql_r(&self, other: &ObjRc, env: &mut Environment) -> BoolResult {
            self.qt_eql_l(other, env)
         }
      }
   }
   macro_rules! impl_defaults {
      (OBJECT; $name:ident ) => {
         fn obj_type(&self) -> ObjType { ObjType::$name(self) }
         fn source(&self) -> Vec<SingleCharacter> {
            let mut ret = vec![];
            for chr in self.to_string().chars(){
               ret.push(SingleCharacter::new(chr));
            }
            ret
         }
      };
      (DISPLAY_DEBUG; $name:ty, $chr:expr) => {
         use std::fmt::{Debug, Formatter, Error, Display};
         impl Display for $name{
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
               write!(f, "{}", self.to_string())
            }
         }

         impl Debug for $name{
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
               write!(f, "{}({})", $chr, self)
            }
         }
      }
   }

   macro_rules! qt_try {
      ($val:expr, $($err:ident => $res:expr),+) => {{
         use result::ObjError;
         match $val {
            Ok(obj) => obj,
            $(Err(ObjError::$err) => $res,)+
            Err(err) => panic!("Unknown error: {:?}", err)
         }
      }};
   }
   macro_rules! peek_char { /* maybe this goes along with qt_try */
      ($env:ident, $($err:ident => $res:expr),+) => {{
         use result::ObjError;
         match $env.stream.peek_char() {
            Ok(obj) => obj.char_val,
            $(Err(ObjError::$err) => $res,)+
            Err(err) => panic!("Unknown error: {:?}", err)
         }
      }};
      ( $env:ident ) => {
         peek_char!($env, EndOfFile => return PluginResponse::NoResponse)
      }
   }

   macro_rules! ok_rc {
      ( $res:expr ) => {{
         use std::rc::Rc;
         Ok(rc!($res))
      }};
      (RESP; $res:expr ) => {{
         use plugins::plugin::PluginResponse;
         PluginResponse::Response(ok_rc!($res))
      }}
   }
   macro_rules! rc {
       ($imp:expr) => ( Rc::new($imp) )
   }
   macro_rules! map {
      { TYPE; $global_type:ident, $($key:expr => $value:expr),+ } => {
         {
            let mut m = $global_type::new();
            $(
               m.insert(ObjRcWrapper(rc!(Symbol::from($key))), $value);
            )+
            m
         }
      }
   }
}

mod objects;
mod plugins;
mod parser;
mod result;
mod builtins;
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
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new(parser::PluginsVec::new(), parser::BuiltinsMap::new());
   p.add_plugin(plugins::pre_command_plugin::INSTANCE);
   // p.add_plugin(plugins::whitespace_plugin::INSTANCE);
   // p.add_plugin(plugins::number_plugin::INSTANCE);
   // p.add_plugin(plugins::text_plugin::INSTANCE);
   // p.add_plugin(plugins::symbol_plugin::INSTANCE);
   // p.add_plugin(plugins::operator_plugin::INSTANCE);
   // p.add_plugin(plugins::universe_plugin::INSTANCE);
   p.add_builtins(builtins::builtins());
   let text = "\
#[include(Number, Whitespace, Operator, Comment)]
#[include_oper(+)]
1 + 2
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

// use std::io;
// use std::io::File;
// use std::path::Path;
// fn lines_from_file<P>(filename: P) -> Result<io::BufReader<File>, io::Error>
//    where P: AsRef<Path> {
//    let mut file = try!(File::open(filename));
//    Ok(io::BufReader::new(file)))
// }