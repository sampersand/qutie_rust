use env::Environment;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::rc::Rc;
use objects::text::Text;
use objects::number::NumberType;

use stream::Stream;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjError};
use parser::Parser;
use objects::boolean::Boolean;
use objects::boolean;
use std::iter::FromIterator;

pub type StackType = Vec<ObjRc>;
pub type LocalsType = HashMap<ObjRcWrapper, ObjRc>;
pub type GlobalsType = LocalsType;
pub type ParenType = [char; 2];
pub struct Universe {
   pub parens: ParenType,
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: GlobalsType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessType {
   Stack,
   Locals,
   Globals,
   All,
   NonStack
}

/* initializer and representation */
impl Universe {
   pub fn new(parens: Option<ParenType>,
              stack: Option<StackType>,
              locals: Option<LocalsType>,
              globals: Option<GlobalsType>) -> Universe {
      Universe{
         parens: 
            if let Some(obj) = parens {
               obj
            } else {
               ['<', '>']
            },
         stack: 
            if let Some(obj) = stack {
               obj
            } else {
               StackType::new()
            },
         locals: 
            if let Some(obj) = locals {
               obj
            } else {
               LocalsType::new()
         },
         globals:
            if let Some(obj) = globals {
               obj
            } else {
               GlobalsType::new()
            }
      }
   }
   pub fn to_string(&self) -> String {
      self.parens[0].to_string() +
      self.to_stream().to_raw_string().as_str() +
      self.parens[1].to_string().as_str()
      // panic!("TODO: TO_STRING FOR UNIVERSE");
   }

   pub fn parse_str(input: &str) -> StackType {
      let mut stack = StackType::new();
      for c in input.chars() {
         stack.push(rc!(SingleCharacter::new(c)))
      }
      stack
   }

   pub fn to_globals(&self) -> Universe {
      let mut globals = self.globals.clone();
      globals.extend(self.locals.clone());
      Universe::new(Some(self.parens), None, None, Some(globals))
   }
   pub fn stream_clone(&self) -> Universe {
      Universe::new(Some(self.parens.clone()),
                    Some(Universe::parse_str(self.to_stream().to_raw_string().as_str())),
                    None,
                    None
                    )
   }
   fn to_stream(&self) -> Stream {
      let mut stream_acc = String::new();
      for item in &self.stack {
         stream_acc.push(cast_as!(item, SingleCharacter).char_val);
      }
      Stream::from_str(stream_acc.as_str())
   }
}

/* Use as a stream */
impl Universe {
   pub fn pop(&mut self) -> ObjResult {
      if let Some(obj) = self.stack.pop() {
         Ok(obj)
      } else {
         Err(ObjError::EndOfFile)
      }
   }

   pub fn push(&mut self, other: ObjRc) {
      self.stack.push(other);
   }
}
/* Use as an Object */
impl Universe {
   fn get_atype(&self, key: &ObjRc, a_type: AccessType) -> AccessType {
      match a_type {
         AccessType::All => 
            if let ObjType::Number(_) = key.obj_type(){
               AccessType::Stack
            } else {
               self.get_atype(key, AccessType::NonStack)
            },
         AccessType::NonStack =>
            if self.locals.contains_key(&rc_wrap!(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         o @ _ => o
      }
   }
   pub fn get(&self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      match self.get_atype(&key, a_type) {
         AccessType::Stack => 
            if let Some(obj) = self.stack.get(cast_as!(key, Number).num_val as usize) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key))
            },
         AccessType::Locals => 
            if let Some(obj) = self.locals.get(&rc_wrap!(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         AccessType::Globals => 
            if let Some(obj) = self.globals.get(&rc_wrap!(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         o @ _ => panic!("Unknown a_type: {:?}", o)
      }
   }

   pub fn set(&mut self, key: ObjRc, val: ObjRc, a_type: AccessType) {
      match self.get_atype(&key, a_type) {
         AccessType::Stack => {
            let pos = cast_as!(key, Number).num_val as isize;
            let stack_len = self.stack.len();
            let pos: usize = if pos < 0 { stack_len as isize + pos }
                             else { pos } as usize;
            if pos > stack_len {
               for i in stack_len..(pos - 1) {
                  self.stack.push(rc!(boolean::NULL))
               }
               self.stack.push(val);
            } else {
               self.stack.push(val);
               if pos != stack_len {
                  self.stack.swap_remove(pos);
               }
            }
         },
         AccessType::Locals => { self.locals.insert(rc_wrap!(key), val); },
         AccessType::Globals => { self.globals.insert(rc_wrap!(key), val); },
         o @ _ => panic!("Shouldn't be trying to set type: {:?}", o)
      };
   }
   pub fn del(&mut self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      let ret = match a_type {
         AccessType::Locals => self.locals.remove(&rc_wrap!(key)),
         AccessType::Globals => self.globals.remove(&rc_wrap!(key)),
         _ => unimplemented!()
      };
      if let Some(obj) = ret {
         Ok(obj)
      } else {
         Err(ObjError::NoSuchKey(key_clone))
      }
   }

   pub fn call(&self, args: ObjRc, env: &mut Environment, do_pop: bool) -> ObjResult {
      if let ObjType::Universe(uni) = args.obj_type() {
         let mut new_universe = uni.to_globals();
         let mut stream = &mut self.to_stream();

         use objects::symbol::Symbol;
         new_universe.locals.insert(rc_wrap!(rc!(Symbol::from("__args"))),
                               args.clone());
         {
            let cloned_env = env.parser.clone();
            let mut stream = &mut env.fork(Some(stream), Some(&mut new_universe), None);
            cloned_env.parse(stream);
         }

         if do_pop {
            if let Some(obj) = new_universe.stack.pop() {
               Ok(obj)
            } else {
               Ok(rc!(boolean::NULL))
            }
         } else {
            Ok(rc!(new_universe))
         }
      } else {
         panic!("Can only call universes with other universes, not: {:?}", args.obj_type());
      }
   }
}

/* QT things */
impl Object for Universe {
   impl_defaults!(OBJECT; Universe);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_TO_BOOL; (|me: &Universe| me.stack.is_empty() && me.locals.is_empty() ));

   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      let mut new_universe = env.universe.to_globals();
      let mut new_stream = self.to_stream();
      {
         let cloned_env = env.parser.clone();
         let mut new_env = &mut env.fork(Some(&mut new_stream), Some(&mut new_universe), None);
         cloned_env.parse(new_env);
      }
      ok_rc!(new_universe)
   }

   fn qt_get(&self, key: ObjRc, a_type: AccessType, _: &mut Environment) -> ObjResult {
      self.get(key, a_type)
   }

   fn qt_set(&mut self, key: ObjRc, val: ObjRc, a_type: AccessType, _: &mut Environment) -> ObjResult {
      let val_clone = val.clone();
      self.set(key, val, a_type);
      Ok(val_clone)
   }


   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      self.call(args, env, true)
   }
}


impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "--[ Stack ]--\n");
      for (i, ele) in self.stack.iter().enumerate() {
         write!(f, "\t{:?}. {:?}\n", i, ele);
      }
      // write!(f, "--[ Locals ]--\n");
      // for (key, val) in self.locals.iter() {
      //    match val.obj_type(){
      //       ObjType::Operator(_) => {},
      //       _ => { write!(f, "\t{:?}: {:?}\n", key, val); }
      //    };
      // }
      Ok(())
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      try!(write!(f, "U("));
      if self.stack.len() > 5 {
         try!(write!(f, "[...], "))
      } else {
         try!(write!(f, "{:?}, ", self.stack))
      }
      use std::iter::Iterator;
      let tmp = self.locals.clone();
      let locals = tmp.values().filter(|v| match v.obj_type(){ObjType::Operator(_)=>false,_=>true});

      if self.locals.len() > 5 {
         try!(write!(f, "{{ ... }}"))
      } else {
         try!(write!(f, "{:?}", self.locals))
      }
      write!(f, ")")
   }
}














