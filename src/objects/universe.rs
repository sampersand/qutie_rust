use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::{ptr, mem};

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use result::{ObjError};

pub type StackType = Vec<ObjBox>;
pub type LocalsType = HashMap<ObjBox, ObjBox>;
pub type GlobalsType = LocalsType;

pub struct Universe {
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: GlobalsType,
}

pub enum AccessType {
   Stack,
   Locals,
   Globals,
}

impl Universe {
   pub fn new() -> Universe {
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: GlobalsType::new(),
      }
   }
   pub fn feed(&mut self, other: ObjBox) {
      self.stack.insert(0, other);
   }

   pub fn next(&mut self) -> Result<ObjBox, ObjError> {
      match self.stack.len() {
         0 => Err(ObjError::EndOfFile),
         _ => Ok(self.stack.remove(0))
      }
   }

   pub fn pop(&mut self) -> Result<ObjBox, ObjError> {
      match self.stack.pop() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile),
      }
   }

   pub fn peek(&self) -> Result<&ObjBox, ObjError> { // aka Result<ObjBox, ObjError> w/ a reference
      match self.stack.first() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile)
      }
   }
   pub fn peek_char(&self) -> Result<&SingleCharacter, ObjError> { // aka Result<ObjBox, ObjError> w/ a reference
      match self.peek() {
         Ok(obj) => match obj.obj_type() {
            ObjType::SingleCharacter(e) => Ok(e),
            otype @ _ => panic!("Don't know how to handle ObjType: {:?}", otype)
         },
         Err(err) => Err(err),
      }
   }

   pub fn push(&mut self, other: ObjBox) {
      self.stack.push(other);
   }

   pub fn get(&self, key: ObjBox, access_type: AccessType) -> Result<&ObjBox, ObjError> {
      match access_type {
         AccessType::Locals => match self.locals.get(&key) {
            Some(obj) => Ok(obj),
            None => panic!("Key `{:?}`, doesn't exist. Do we return null or panic?", key)
         },
         _ => unimplemented!()
      }
   }

   pub fn set(&mut self, key: ObjBox, val: ObjBox, access_type: AccessType) {
      match access_type {
         AccessType::Locals => self.locals.insert(key, val),
         _ => unimplemented!()
      };
   }
}
#[derive(Debug, PartialEq, Eq)]
struct DeleteMe(i32);

impl Object for Universe {
   fn obj_type(&self) -> ObjType { ObjType::Universe }
   
   fn source(&self) -> Vec<SingleCharacter>{
      unimplemented!();
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
      // write!(f, "U([");
      // if 0 < self.stack.len() {
      //    Display::fmt(&self.stack[0], f);
      //    let mut pos = 1;
      //    while pos < self.stack.len(){ // TODO: FOR LOOPS
      //       // write!(f, "|");
      //       Display::fmt(&self.stack[pos], f);
      //       pos += 1;
      //    }
      // }
      // write!(f, "]");
      // write!(f, ")")
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U({:?}, {:?})", self.stack, self.locals)
   }
}








