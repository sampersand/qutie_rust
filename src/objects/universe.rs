use env::Environment;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::rc::Rc;
use objects::text::Text;
use objects::number::NumberType;

use stream::Stream;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjError};
use parser::Parser;
use objects::boolean::{Boolean, BoolType};
use objects::number::Number;
use objects::symbol::Symbol;
use std::iter::FromIterator;

pub type StackType = Vec<ObjRc>;
pub type LocalsType = HashMap<ObjRcWrapper, ObjRc>;
pub type GlobalsType = LocalsType;
pub struct Universe {
   pub parens: [char; 2],
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: GlobalsType,
   rc: Option<Rc<Universe>>,
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
   pub fn new(parens: Option<[char; 2]>,
              stack: Option<StackType>,
              locals: Option<LocalsType>,
              globals: Option<GlobalsType>) -> Universe {
      Universe{
         parens: 
            if let Some(obj) = parens { obj } 
            else { ['<', '>'] },
         stack: 
            if let Some(obj) = stack { obj }
            else { StackType::new() },
         locals: 
            if let Some(obj) = locals { obj } 
            else { LocalsType::new() },
         globals:
            if let Some(obj) = globals { obj }
            else { GlobalsType::new() },
         rc: None
      }
   }

   pub fn to_rc(mut self) -> Rc<Universe>{
      let ret = Rc::new(self);
      self.rc = Some(ret.clone());
      ret
   }

   pub fn to_string(&self) -> String {
      let mut ret = self.parens[0].to_string();
      if let Some(stream) = self.to_stream(){
         ret.push_str(stream.to_raw_string().as_str());
      } else {
         for obj in &self.stack{
            ret.push_str(obj.to_string().as_str());
            ret.push_str(", ");
         }
         if !self.stack.is_empty() {
            assert_eq!(ret.pop().unwrap(), ' ');
            assert_eq!(ret.pop().unwrap(), ',');
         }
         for (key, val) in self.locals.iter() {
            ret.push_str(key.0.to_string().as_str());
            ret.push_str(": ");
            ret.push_str(val.to_string().as_str());
            ret.push_str(", ");
         }
         if !self.locals.is_empty() {
            assert_eq!(ret.pop().unwrap(), ' ');
            assert_eq!(ret.pop().unwrap(), ',');
         }
      }
      ret.push(self.parens[1]);
      ret
   }

   pub fn parse_str(input: &str) -> StackType {
      let mut stack = StackType::new();
      for c in input.chars() {
         stack.push(SingleCharacter::new(c).to_rc())
      }
      stack
   }

   pub fn to_globals(&self) -> Universe {
      let mut globals = self.globals.clone();
      globals.extend(self.locals.clone());
      Universe::new(Some(self.parens), None, None, Some(globals))
   }

   fn to_stream(&self) -> Option<Stream> {
      let mut stream_acc = String::new();
      for item in &self.stack {
         // here's the issue; we're casting a number into a SingleCharacter,
         // which is translating "38" into "&"
         if !item.is_a(ObjType::SingleCharacter) {
            return None;
         }
         stream_acc.push(cast_as!(CL; item, SingleCharacter).char_val);
      }
      Some(Stream::from_str(stream_acc.as_str()))
   }

   fn get_rc(&self) -> Rc<Universe> {
      self.rc.clone().expect("No rc object found")
   }
}

/* Use as a stack */
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

/* Accessors */
impl Universe {
   fn get_atype(&self, key: &ObjRc, a_type: AccessType) -> AccessType {
      match a_type {
         AccessType::All => 
            if key.is_a(ObjType::Number) {
               AccessType::Stack
            } else {
               self.get_atype(key, AccessType::NonStack)
            },
         AccessType::NonStack =>
            if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
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
            if let Some(obj) = self.stack.get(cast_as!(CL; key, Number).num_val as usize) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key))
            },
         AccessType::Locals => 
            if let Some(obj) = self.locals.get(&ObjRcWrapper(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         AccessType::Globals => 
            if let Some(obj) = self.globals.get(&ObjRcWrapper(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         o @ _ => panic!("Unknown a_type: {:?}", o)
      }
   }

   pub fn set(&mut self, key: ObjRc, val: ObjRc, a_type: AccessType) {
      match self.get_atype(&key, a_type) {
         AccessType::Stack => 
            {
               let stack_len = self.stack.len();
               let pos = cast_as!(key, Number).num_val as isize;
               let pos =
                  if pos < 0 {
                     stack_len as isize + pos /* if we have a negative position, invert it */
                  } else {
                     pos
                  } as usize;

               if stack_len < pos { /* if we access an element too far out, add nulls until we get there */
                  for i in stack_len..(pos - 1) {
                     self.stack.push(new_obj!(BOOL_STATIC, Null))
                  }
                  self.stack.push(val);
               } else {
                  self.stack.push(val);
                  if pos != stack_len {
                     self.stack.swap_remove(pos);
                  }
               }
         },
         AccessType::Locals => 
            {
               self.locals.insert(ObjRcWrapper(key), val);
            },
         AccessType::Globals =>
            {
               self.globals.insert(ObjRcWrapper(key), val);
            },
         o @ _ => panic!("Shouldn't be trying to set type: {:?}", o)
      };
   }
   pub fn del(&mut self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      let ret =
         match a_type {
            AccessType::Locals => self.locals.remove(&ObjRcWrapper(key)),
            AccessType::Globals => self.globals.remove(&ObjRcWrapper(key)),
            _ => unimplemented!()
         };
      if let Some(obj) = ret {
         Ok(obj)
      } else {
         Err(ObjError::NoSuchKey(key_clone))
      }
   }

   pub fn call(&self, args: Rc<Universe>, env: &mut Environment, do_pop: bool) -> ObjResult {
      if !args.is_a(ObjType::Universe) {
         panic!("Can only call universes with other universes, not: {:?}", args.obj_type());
      }
      let mut new_universe = args.to_globals();
      let mut stream = &mut self.to_stream().unwrap();

      new_universe.locals.insert(ObjRcWrapper(new_obj!(SYM_STATIC, "__args")), args.clone()); /* add __args in */
      {
         let cloned_env = env.parser.clone();
         let mut stream = &mut env.fork(Some(stream), Some(&mut new_universe), None);
         cloned_env.parse(stream);
      }

      Ok(if do_pop {
            if let Some(obj) = new_universe.stack.pop() { obj }
            else { new_obj!(BOOL_STATIC, Null) }
         } else { Rc::new(new_universe) })
   }
}

/* QT things */
impl Object for Universe {
   impl_defaults!(OBJECT; Universe);

   fn qt_to_text(&self, env: &mut Environment) -> Result<Rc<Text>, ObjError> {
      match get_method!(self.get_rc(), "__text", env) {
         Ok(obj) => Ok(cast_as!(obj.qt_call(env.universe.to_globals().to_rc(), env).unwrap(), Text)),
         Err(_) => Ok(new_obj!(TEXT, self.to_string()))
      }
   }

   obj_functions!(QT_TO_BOOL; (|me: &Universe| me.stack.is_empty() && me.locals.is_empty() ));

   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      let mut new_universe = env.universe.to_globals();
      let mut new_stream = self.to_stream().unwrap();
      let cloned_env = env.parser.clone();
      {
         cloned_env.parse(&mut env.fork(Some(&mut new_stream), Some(&mut new_universe), None));
      }
      Ok(Rc::new(new_universe))
   }

   fn qt_get(&self, key: ObjRc, _: &mut Environment) -> ObjResult {
      self.get(key, AccessType::All)
   }

   fn qt_set(&mut self, key: ObjRc, val: ObjRc, _: &mut Environment) -> ObjResult {
      self.set(key, val.clone(), AccessType::All);
      Ok(val)
   }


   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      self.call(cast_as!(args, Universe), env, true)
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
      //    match val.old_obj_type(){
      //       OldObjType::Operator(_) => {},
      //       _ => { write!(f, "\t{:?}: {:?}\n", key, val); }
      //    };
      // }
      Ok(())
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      try!(write!(f, "U("));
      if self.stack.len() > 10 {
         try!(write!(f, "[...], "))
      } else {
         try!(write!(f, "{:?}, ", self.stack))
      }
      use std::iter::Iterator;
      let tmp = self.locals.clone();
      let locals = tmp.values().filter(|v| !v.is_a(ObjType::Builtin));

      if self.locals.len() > 5 {
         try!(write!(f, "{{ ... }}"))
      } else {
         try!(write!(f, "{:?}", self.locals))
      }
      write!(f, ")")
   }
}














