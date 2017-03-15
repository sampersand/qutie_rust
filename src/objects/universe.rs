use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};

use objects::object::{Object, ObjectType};
use objects::boxed_obj::BoxedObj;
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjError};

pub type StackType = Vec<BoxedObj>;
pub type LocalsType = HashMap<BoxedObj, BoxedObj>;
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
   pub fn feed(&mut self, other: BoxedObj) {
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

   pub fn peek(&self) -> Result<&BoxedObj, ObjError> { // aka ObjResult w/ a reference
      match self.stack.first() {
         Some(obj) => Ok(obj),
         None => Err(ObjError::EndOfFile)
      }
   }
   pub fn peek_char(&self) -> Result<&SingleCharacter, ObjError> { // aka ObjResult w/ a reference
      match self.peek() {
         Ok(obj) => match obj.obj_type() {
            ObjectType::SingleCharacter(e) => Ok(e),
            otype @ _ => panic!("Don't know how to handle ObjectType: {:?}", otype)
         },
         Err(err) => Err(err),
      }
   }

   pub fn push(&mut self, other: BoxedObj) {
      self.stack.push(other);
   }

   // pub fn get(&self, key: BoxedObj, access_type: AccessType) -> Result<&BoxedObj, ObjError> {
   //    match access_type {
   //       AccessType::Locals => match self.locals.get(&key) {
   //          Some(obj) => Ok(obj),
   //          None => panic!("Key doesn't exist. Do we return null or panic?")
   //       },
   //       _ => unimplemented!()
   //    }
   // }
   pub fn get(&self, key: BoxedObj, access_type: AccessType) -> ObjResult {
      match access_type {
         AccessType::Locals => match self.locals.get(&key) {
            Some(obj) => {
               Ok(*obj)
            },
            None => panic!("Key doesn't exist. Do we return null or panic?")
         },
         _ => unimplemented!()
      }
   }
   
   pub fn set(&mut self, key: BoxedObj, val: BoxedObj, access_type: AccessType) -> ObjResult {
      match access_type {
         AccessType::Locals => self.locals.insert(key, val),
         _ => unimplemented!()
      };
      self.get(key, AccessType::Locals)
   }


}

impl Object for Universe {
   fn obj_type(&self) -> ObjectType { ObjectType::Universe }
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








