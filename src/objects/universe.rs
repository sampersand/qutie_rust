use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};

use environment::Environment;
use objects::object::{Object, ObjectType};
use objects::boxed_obj::BoxedObj;
use objects::single_character::SingleCharacter;
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
   pub fn feed(&mut self, other: BoxedObj) {
      self.stack.insert(0, other);
   }

   pub fn next(&mut self) -> Option<BoxedObj> {
      match self.stack.len() {
         0 => None,
         _ => Some(self.stack.remove(0))
      }
   }

   pub fn pop(&mut self) -> Option<BoxedObj> {
      self.stack.pop()
   }

   pub fn peek(&self) -> Option<&BoxedObj> {
      self.stack.first()
   }
   pub fn peek_char(&self) -> Option<&SingleCharacter> {
      match self.stack.first() {
         None => None,
         Some(obj) => match obj.obj_type() {
            ObjectType::SingleCharacter(e) => Some(e),
            e @ _ => panic!("Unknown type {:?}", e)
         }
      }
   }

   pub fn peek_char_amnt(&self, amnt: usize) -> Vec<&SingleCharacter>{
      let mut ret = Vec::<&SingleCharacter>::new();
      ret.push(self.peek_char().unwrap()); // todo this
      ret
   }

   pub fn push(&mut self, other: BoxedObj) {
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
   pub fn spawn_clone_stack(&self) -> Universe{
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: LocalsType::new(),
      }
   }
   pub fn clone(&self) -> Universe {
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: LocalsType::new(),
      }
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
         while pos < self.stack.len(){ // TODO: FOR LOOPS
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








