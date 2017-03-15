use objects::object::{Object, ObjType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;

pub struct Null;


impl Null {
   pub fn new() -> Null{
      Null{}
   }
}

impl Object for Null {
   fn obj_type(&self) -> ObjType { ObjType::Null }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in "null".to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
}


impl Display for Null {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "null")
   }
}
impl Debug for Null {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}












