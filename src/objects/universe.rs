use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};

use objects::object::{Object, ObjectType};
use objects::boxed_obj::BoxedObj;
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjErr};

pub type StackType = Vec<BoxedObj>;
pub type LocalsType = HashMap<BoxedObj, BoxedObj>;
pub type GlobalsType = LocalsType;

pub struct Universe {
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: GlobalsType,
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
         globals: GlobalsType::new(),
      }
   }
   pub fn feed(&mut self, other: BoxedObj) {
      self.stack.insert(0, other);
   }

   pub fn next(&mut self) -> ObjResult {
      match self.stack.len() {
         0 => Err(ObjErr::EndOfFile),
         _ => Ok(self.stack.remove(0))
      }
   }

   pub fn pop(&mut self) -> ObjResult {
      match self.stack.pop() {
         Some(obj) => Ok(obj),
         None => Err(ObjErr::EndOfFile),
      }
   }

   pub fn peek(&self) -> Result<&BoxedObj, ObjErr> { // aka ObjResult w/ a reference
      match self.stack.first() {
         Some(obj) => Ok(obj),
         None => Err(ObjErr::EndOfFile)
      }
   }
   pub fn peek_char(&self) -> Result<&SingleCharacter, ObjErr> { // aka ObjResult w/ a reference
      match self.peek() {
         Ok(obj) => match obj.obj_type() {
            ObjectType::SingleCharacter(e) => Ok(e),
            e @ _ => panic!("Unknown type {:?}", e)
         },
         Err(err) => Err(err),
      }
   }

   pub fn push(&mut self, other: BoxedObj) {
      self.stack.push(other);
   }
   pub fn get(&self, pos: BoxedObj, access_type: AccessTypes) -> BoxedObj {
      panic!("{:?}", "NO GET RIGHT NOW");
      // match access_type {
      //    AccessTypes::Stack => match pos.obj_type() {
      //       _ => panic!("{:?}", "failure")
      //    },
      //    AccessTypes::Locals => self.locals[&pos],
      //    AccessTypes::Globals => self.globals[&pos],
      // }
   }
   // pub fn spawn_clone_stack(&self) -> Universe{
   //    Universe{
   //       stack: StackType::new(),
   //       locals: LocalsType::new(),
   //       globals: GlobalsType::new(),
   //    }
   // }
   // pub fn clone(&self) -> Universe {
   //    Universe{
   //       stack: StackType::new(),
   //       locals: LocalsType::new(),
   //       globals: GlobalsType::new(),
   //    }
   // }
   // pub fn fork(self, stack: Option<StackType>, locals: Option<LocalsType>, globals: Option<GlobalsType>) -> Universe {
   //    Universe {
   //       stack: match stack{ Some(e) => e, None => self.stack },
   //       locals: match locals{ Some(e) => e, None => self.locals },
   //       globals: match globals{ Some(e) => e, None => self.globals }
   //    }
   // }
}

impl Object for Universe {
   fn obj_type(&self) -> ObjectType { ObjectType::Universe }
   fn source(&self) -> Vec<SingleCharacter>{
      unimplemented!();
   }
}

impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U([");
      if 0 < self.stack.len() {
         Display::fmt(&self.stack[0], f);
         let mut pos = 1;
         while pos < self.stack.len(){ // TODO: FOR LOOPS
            // write!(f, "|");
            Display::fmt(&self.stack[pos], f);
            pos += 1;
         }
      }
      write!(f, "]");
      write!(f, ")")
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "U({:?}, {:?})", self.stack, self.locals)
   }
}








