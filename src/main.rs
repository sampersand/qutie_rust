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
            ok_rc!(Text::new(self.to_string(), None))
         }
      };
      (QT_EQL; $old_obj_type:ident, $comp_item:ident) => {
         fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> ObjResult {
            let other = match other.old_obj_type() {
               OldObjType::$old_obj_type(ele) => ele,
               // _ => return Err(ObjError::NotImplemented)
               _ => return ok_rc!(Boolean::from_bool(false))
            };
            ok_rc!(Boolean::from_bool(self.$comp_item == other.$comp_item))
         }
         fn qt_eql_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
            self.qt_eql_l(other, env)
         }
      };
      (QT_METHODS; $obj_mod:ident) => {
         fn qt_method(&self, meth: &str, env: &mut Environment) -> ObjResult {
            use objects::methods::$obj_mod;
            $obj_mod::get_method(self, meth, env)
         }
      }
   }
   macro_rules! impl_defaults {
      (OBJECT; $name:ident ) => {
         fn obj_type(&self) -> ObjType { ObjType::$name }
         fn is_a(&self, obj_type: ObjType) -> bool { self.obj_type() == obj_type }

         fn old_obj_type(&self) -> OldObjType { OldObjType::$name(self) }
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
   macro_rules! looked { 
      ($env:ident, $res:expr) => {
         if let Some(obj) = $env.stream.looked() {
            *obj
         } else {
            $res
         }
      };
      ($env:ident, $guard:ident, $default:expr) => {
         match $env.stream.looked() {
            Some(c) if $guard(*c) => *c,
            _ => $default
         }
      };
      ( $env:ident ) => {
         looked!($env, return PluginResponse::NoResponse)
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
   macro_rules! rc_wrap {
       ($imp:expr) => ( ObjRcWrapper($imp) )
   }

   macro_rules! map {
      { TYPE; $global_type:ident, $($key:expr => $value:expr),+ } => {
         {
            let mut m = $global_type::new();
            $(
               m.insert(rc_wrap!(rc!(Symbol::from($key))), $value);
            )+
            m
         }
      }
   }
   macro_rules! cast_as {
       ($from:expr, $to:ident) => {
         match $from.old_obj_type() {
            OldObjType::$to(obj) => obj,
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
mod stream;


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


