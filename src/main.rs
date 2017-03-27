#![allow(unused)]

#[macro_use]
extern crate guard;

#[macro_use]
extern crate lazy_static;

extern crate regex;

#[macro_use]
mod qt_macros {
   macro_rules! obj_functions {
      (QT_TO_BOOL; $bool_expr:expr) => {
         fn qt_to_bool(&self, _: &mut Environment) -> Result<Rc<Boolean>, ObjError> {
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
         fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> ObjResult {
            let other = match other.obj_type() {
               ObjType::$obj_type(ele) => ele,
               // _ => return Err(ObjError::NotImplemented)
               _ => return ok_rc!(Boolean::from_bool(false))
            };
            ok_rc!(Boolean::from_bool(self.$comp_item == other.$comp_item))
         }
         fn qt_eql_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
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
      ($val:expr $(,$err:ident => $res:expr)*) => {{
         use result::ObjError;
         match $val {
            Ok(obj) => obj,
            $(Err(ObjError::$err) => $res,)*
            Err(err) => panic!("Unknown error: {:?}", err)
         }
      }};
   }
   macro_rules! peek_char { 
      ($env:ident, $res:expr) => {
         if let Some(obj) = $env.stream.peek_char() {
            obj
         } else {
            $res
         }
      };
      ($env:ident, $guard:ident, $default:expr) => {
         match $env.stream.peek_char() {
            Some(c) if $guard(c) => c,
            _ => $default
         }
      };
      ( $env:ident ) => {
         peek_char!($env, return PluginResponse::NoResponse)
      }
   }

   macro_rules! assert_next_eq {
       ($lhs:expr, $env:expr) => {{
         use objects::object::ObjType;
         assert_eq!($lhs, cast_as!($env.stream.next().unwrap(), SingleCharacter).char_val);
       }}
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
   macro_rules! cast_as {
       ($from:expr, $to:ident) => {
         match $from.obj_type() {
            ObjType::$to(obj) => obj,
            other @ _ => panic!("Unexpected type: {:?}", other)
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


fn main() {

   const BUFFER_SIZE: usize = 1 << 16;
   let inp_file = "/Users/westerhack/code/rust/qutie_rust/examples/example.qt";

   use std::fs::File;
   use std::io::Read;

   let mut text = String::new();

   match File::open(inp_file) {
      Ok(file) => file,
      Err(err) => panic!("Cannot open file {:?} for reading: {}", inp_file, err)
   }.read_to_string(&mut text);

   println!("====[ Runtime ]====");
   let mut p = parser::Parser::new(parser::PluginsVec::new());
   
   p.add_plugin(plugins::pre_command_plugin::INSTANCE);

   let r = p.process(text.as_str());
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