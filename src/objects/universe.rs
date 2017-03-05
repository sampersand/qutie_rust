use std::collections::HashMap;
use objects::Object;
use objects::BoxObj;


pub type StackType = Vec<BoxObj>;
pub type LocalsType = HashMap<BoxObj, BoxObj>;


pub struct Universe {
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: LocalsType,
}

use std;

impl Universe {
   pub fn new() -> Universe {
      Universe{
         stack: StackType::new(),
         locals: LocalsType::new(),
         globals: LocalsType::new(),
      }
   }
   pub fn push(&mut self, other: BoxObj) {
      self.stack.push(other);
   }
}
impl Object for Universe {}
use std::fmt::{Debug, Formatter, Error};

impl Debug for Universe{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
      write!(f, "Universe{{ {:?}, {:?} }}", self.stack, self.locals);
      Ok(())
   }
}





