use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};

use environment::Environment;
use objects::object::{Object, ObjectType};
use objects::boxed_obj::BoxedObj;
use objects::number::Number;

pub type StackType = Vec<BoxedObj>;
pub type LocalsType = HashMap<BoxedObj, BoxedObj>;

pub struct Universe {
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: LocalsType,
}

pub enum AccessTypes {
   Stack,
   Locals,
   Globals,
}

impl Universe {
   pub fn new() -> Universe {
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: LocalsType::new(),
      }
   }
   pub fn feed(&mut self, other: BoxedObj, _: &mut Environment) {
      self.stack.insert(0, other);
   }

   pub fn next(&mut self, _: &mut Environment) -> Option<BoxedObj> {
      match self.stack.len() {
         0 => None,
         _ => Some(self.stack.remove(0))
      }
   }
   pub fn push(&mut self, other: BoxedObj, _: &mut Environment) {
      self.stack.push(other);
   }
   pub fn get(&self, pos: BoxedObj, access_type: AccessTypes) -> BoxedObj {
      panic!("{:?}", "BAD");
      // match access_type {
      //    AccessTypes::Stack => match pos.obj_type() {
      //       _ => panic!("{:?}", "failure")
      //    },
      //    AccessTypes::Locals => self.locals[&pos],
      //    AccessTypes::Globals => self.globals[&pos],
      // }
   }
}

impl Object for Universe {
   fn obj_type(&self) -> ObjectType { ObjectType::Universe }
}

impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U([");
      if 0 < self.stack.len() {
         Display::fmt(&self.stack[0], f);
         let mut pos = 1;
         while pos < self.stack.len(){
            write!(f, ", ");
            Display::fmt(&self.stack[pos], f);
            pos += 1;
         }
      }
      write!(f, "]{{TODO}}");
      write!(f, ")")
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U({:?}, {:?})", self.stack, self.locals)
   }
}








