use env::Environment;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::rc::Rc;
use objects::number::NumberType;

use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjError};
use parser::Parser;

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

#[derive(Debug)]
pub enum AccessType {
   Stack,
   Locals,
   Globals,
   All,
   NonStack
}

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
   pub fn parse_str(input: &str) -> StackType {
      let mut stack = StackType::new();
      for c in input.chars() {
         stack.push(Rc::new(SingleCharacter::new(c)))
      }
      stack
   }
   pub fn feed(&mut self, other: ObjRc) {
      self.stack.insert(0, other);
   }

   pub fn next(&mut self) -> ObjResult {
      match self.stack.len() {
         0 => Err(ObjError::EndOfFile),
         _ => Ok(self.stack.remove(0))
      }
   }

   pub fn pop(&mut self) -> ObjResult {
      match self.stack.pop() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile),
      }
   }

   pub fn peek(&self) -> Result<&ObjRc, ObjError> { // aka ObjResult w/ a reference
      match self.stack.first() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile)
      }
   }
   pub fn peek_char(&self) -> Result<&SingleCharacter, ObjError> { // aka ObjResult w/ a reference
      match self.peek() {
         Ok(obj) => match obj.obj_type() {
            ObjType::SingleCharacter(e) => Ok(e),
            other @ _ => panic!("Don't know how to handle ObjType: {:?}", other)
         },
         Err(err) => Err(err),
      }
   }

   pub fn push(&mut self, other: ObjRc) {
      self.stack.push(other);
   }

   pub fn get(&self, key: ObjRc, access_type: AccessType) -> ObjResult {
      let access_type = match access_type {
         AccessType::All => match key.obj_type(){
               ObjType::Number(_) => AccessType::Stack,
               _ => AccessType::Locals
         },
         AccessType::NonStack => if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         _ => access_type
      };
      match access_type {
         AccessType::Locals => match self.locals.get(&ObjRcWrapper(key)) {
            Some(obj) => Ok(obj.clone()),
            None => panic!("Key doesn't exist. Do we return null or panic?")
         },
         AccessType::Globals => match self.globals.get(&ObjRcWrapper(key)) {
            Some(obj) => Ok(obj.clone()),
            None => panic!("Key doesn't exist. Do we return null or panic?")
         },
         _ => panic!("Unknown access_type: {:?}", access_type)
      }
   }

   pub fn set(&mut self, key: ObjRc, val: ObjRc, access_type: AccessType) -> ObjResult {
      match access_type {
         AccessType::Locals => {
            let ret = val.clone();
            self.locals.insert(ObjRcWrapper(key), val);
            Ok(ret)
         },
         _ => unimplemented!()
      }
   }
   fn to_globals(&self) -> Universe {
      let mut globals = self.globals.clone();
      globals.extend(self.locals.clone());
      Universe::new(Some(self.parens), None, None, Some(globals))
   }
}

impl Object for Universe {
   fn obj_type(&self) -> ObjType { ObjType::Universe(self) }
   
   fn source(&self) -> Vec<SingleCharacter>{
      println!("{:?}", "unimplemented universe source");
      unimplemented!();
   }

   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      let mut new_env = self.to_globals();
      let mut new_stream = Universe::new(None, Some(self.stack.as_slice().to_vec()), None, None);
      {
         env.parser.parse(&mut env.fork(Some(&mut new_stream), Some(&mut new_env), None));
      }
      Ok(Rc::new(new_env))
   }
   fn qt_get(&self, rhs: ObjRc, access_type: AccessType, env: &mut Environment) -> ObjResult {
      let access_type = match access_type {
         AccessType::All => match rhs.obj_type(){
               ObjType::Number(_) => AccessType::Stack,
               _ => AccessType::Locals
         },
         AccessType::NonStack => if self.locals.contains_key(&ObjRcWrapper(rhs.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         _ => access_type
      };
      match access_type {
         AccessType::Stack => {
            let num_val = match rhs.qt_to_num(env) {
               Ok(obj) => obj,
               _ => panic!("Cannot convert `{:?}` to number", rhs)
            }.num_val;
            Ok(self.stack.get(num_val as usize).unwrap().clone())
         },
         AccessType::Locals => {
            let obj_wrapper = &ObjRcWrapper(rhs);
            match self.locals.get(obj_wrapper) {
               Some(obj) => Ok(obj.clone()),
               None => panic!("Bad key")
            }
         }
         other @ _ => panic!("Unhandled AccessType: {:?}", other)
      }
   }
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      match args.obj_type() {
         ObjType::Universe(uni) => {
            let mut new_env = uni.to_globals();
            let mut stack = &mut Universe::new(Some(self.parens), Some(self.stack.clone()), None, None);
            {
               let mut stream = &mut Environment::new(stack, &mut new_env, env.parser);
               env.parser.parse(stream);
            }
            Ok(Rc::new(new_env))
         },
         other @ _ => panic!("Cant call universe with type: {:?}", other)
      }
   }
}


impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "--[ Stack ]--\n");
      for (i, ele) in self.stack.iter().enumerate() {
         write!(f, "\t{}. {:?}\n", i, ele);
      }
      write!(f, "--[ Locals ]--\n");
      for (key, val) in self.locals.iter() {
         write!(f, "\t{}: {:?}\n", key, val);
      }
      Ok(())
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U({:?}, {:?}, {:?}, {:?})", self.parens, self.stack, self.locals, self.globals)
   }
}














