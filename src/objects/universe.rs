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

#[derive(Debug, Clone, PartialEq)]
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
         parens: match parens {
            Some(obj) => obj,
            None => ['<', '>']
         },
         stack: match stack {
            Some(obj) => obj, 
            None => StackType::new()
         },
         locals: match locals {
            Some(obj) => obj,
            None => LocalsType::new(),
         },
         globals: match globals {
            Some(obj) => obj,
            None => GlobalsType::new(),
         },
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
}

/* Use as a stream */
impl Universe {
   pub fn pop(&mut self) -> ObjResult {
      match self.stack.pop() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile),
      }
   }

   pub fn push(&mut self, other: ObjRc) {
      self.stack.push(other);
   }
}
/* Use as an Object */
impl Universe {
   pub fn get(&self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let a_type = match a_type {
         AccessType::All => match key.obj_type(){
            ObjType::Number(_) => AccessType::Stack,
            _ => if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         },
         AccessType::NonStack => if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         _ => a_type
      };
      let key_clone = key.clone();
      match a_type {
         AccessType::Stack => {
            let num_val = cast_as!(key, Number).num_val;
            match self.stack.get(num_val as usize) {
               Some(obj) => Ok(obj.clone()),
               None => Err(ObjError::NoSuchKey(key_clone))
            }
         },
         AccessType::Locals => match self.locals.get(&ObjRcWrapper(key)) {
            Some(obj) => Ok(obj.clone()),
            None => Err(ObjError::NoSuchKey(key_clone))
         },
         AccessType::Globals => match self.globals.get(&ObjRcWrapper(key)) {
            Some(obj) => Ok(obj.clone()),
            None => Err(ObjError::NoSuchKey(key_clone))
         },
         _ => panic!("Unknown a_type: {:?}", a_type)
      }
      /*
      /* this is bad */
      let a_type = match a_type {
         AccessType::All => match key.obj_type(){
            ObjType::Number(num) if 0 <= num.num_val && num.num_val < self.stack.len() as i32 => AccessType::Stack,
            _ => if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         },
         AccessType::NonStack => if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         _ => a_type
      };
      let key_clone = key.clone();
      match a_type {
         AccessType::Stack => {
            let num_val = match key.qt_to_num(env) {
               Ok(obj) => obj,
               _ => panic!("Cannot convert `{:?}` to number", key)
            }.num_val;
            match self.stack.get(num_val as usize) {
               Some(obj) => Ok(obj.clone()),
               None => Err(ObjError::NoSuchKey(key_clone))
            }
         },
         AccessType::Locals => {
            // if let ObjType::Symbol(sym) = key.obj_type() {
            //     if sym.sym_val == "__self" {
            //       return Ok(rc!(self.fork()))
            //    }
            // }
            let obj_wrapper = &ObjRcWrapper(key);
            match self.locals.get(obj_wrapper) {
               Some(obj) => Ok(obj.clone()),
               None => Err(ObjError::NoSuchKey(key_clone))
            }
         },
         AccessType::Globals => {
            let obj_wrapper = &ObjRcWrapper(key);
            match self.globals.get(obj_wrapper) {
               Some(obj) => Ok(obj.clone()),
               None => Err(ObjError::NoSuchKey(key_clone))
            }
         }
         other @ _ => panic!("Unhandled AccessType: {:?}", other)
      }
      */
   }

   pub fn set(&mut self, key: ObjRc, val: ObjRc, a_type: AccessType) -> ObjResult {
      let a_type = match a_type {
                      AccessType::All => match key.obj_type() {
                                           ObjType::Number(num) => AccessType::Stack,
                                           _ => AccessType::Locals
                                         },
                      o @ _ => o
                   };
      match a_type {
         AccessType::Stack => {
            let val_clone = val.clone();
            let pos = cast_as!(key, Number).num_val as isize;
            let stack_len = self.stack.len();
            let pos: usize = if pos < 0 { stack_len as isize + pos }
                             else { pos } as usize;
            if pos > stack_len + 1 {
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
            Ok(val_clone)
         },
         AccessType::Locals => {
            let val_clone = val.clone();
            self.locals.insert(ObjRcWrapper(key), val);
            Ok(val_clone)
         },
         AccessType::Globals => {
            let ret = val.clone();
            self.globals.insert(ObjRcWrapper(key), val);
            Ok(ret)
         },
         _ => panic!("Shouldn't be trying to set type: {:?}", a_type)

      }
   }
   pub fn del(&mut self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      let ret = match a_type {
         AccessType::Locals => self.locals.remove(&ObjRcWrapper(key)),
         AccessType::Globals => self.globals.remove(&ObjRcWrapper(key)),
         _ => unimplemented!()
      };
      match ret {
         Some(obj) => Ok(obj),
         None => Err(ObjError::NoSuchKey(key_clone))
      }
   }
   fn to_stream(&self) -> Stream {
      let mut stream_acc = String::new();
      for item in &self.stack {
         stream_acc.push(cast_as!(item, SingleCharacter).char_val);
      }
      Stream::from_str(stream_acc.as_str())
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
      self.set(key, val, a_type)
   }


   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      match args.obj_type() {
         ObjType::Universe(uni) => {
            let mut new_env = uni.to_globals();
            let mut stream = &mut self.to_stream();
            use objects::symbol::Symbol;
            new_env.locals.insert(ObjRcWrapper(rc!(Symbol::from("__args"))), args.clone());
            {
               let cloned_env = env.parser.clone();
               let mut stream = &mut env.fork(Some(stream), Some(&mut new_env), None);
               cloned_env.parse(stream);
            }
            match new_env.stack.pop() {
               Some(obj) => Ok(obj),
               None => Ok(rc!(boolean::NULL))
            }
         },
         other @ _ => panic!("Cant call universe with type: {:?}", other)
      }
   }
}


impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "--[ Stack ]--\n");
      for (i, ele) in self.stack.iter().enumerate() {
         write!(f, "\t{:?}. {:?}\n", i, ele);
      }
      write!(f, "--[ Locals ]--\n");
      for (key, val) in self.locals.iter() {
         match val.obj_type(){
            ObjType::Operator(_) => {},
            _ => { write!(f, "\t{:?}: {:?}\n", key, val); }
         };
      }
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














