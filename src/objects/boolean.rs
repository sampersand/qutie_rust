use objects::object::{Object, ObjType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;

pub enum Boolean {
   True,
   False,
   Null
}

impl Boolean {
   pub fn to_bool(&self) -> bool{
      match *self {
         Boolean::True => true,
         Boolean::False => false,
         Boolean::Null => false,
      }
   }
   pub fn from_bool(inp: bool) -> Boolean {
      match inp {
         true => Boolean::True,
         false => Boolean::False
      }
   }
}

impl Object for Boolean {
   fn obj_type(&self) -> ObjType { ObjType::Boolean }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.to_bool().to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
}


impl Display for Boolean {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "null")
   }
}
impl Debug for Boolean {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}












