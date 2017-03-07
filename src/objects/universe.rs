use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error};
use std::process;

use environment::Environment;
use objects::Object;
use objects::BoxedObj;
use util;


pub type StackType = Vec<BoxedObj>;
pub type LocalsType = HashMap<BoxedObj, BoxedObj>;


pub struct Universe {
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: LocalsType,
}


impl Universe {
   pub fn new() -> Universe {
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: LocalsType::new(),
      }
   }
   pub fn push(&mut self, other: BoxedObj) {
      self.stack.push(other);
   }

   pub fn next(&mut self, env: &mut Environment) -> BoxedObj {
      match self.stack.pop() {
         Some(e) => e,
         None => util::exit(1)
      }
   }
}

impl Object for Universe {}

impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "Universe{{ {:?}, {:?} }}", self.stack, self.locals);
      Ok( () )
   }
}





















