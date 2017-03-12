use objects::object::{Object, ObjectType};

type SourceType = char;

pub struct SingleCharacter {
   pub source_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{source_val: inp}
   }
   fn get_char(&self) -> SourceType {
      match self.source_val{
         ' ' => '_',
         '_' => '–',
         e @ _ => e
      }
   }
}

impl Object for SingleCharacter{
   fn obj_type(&self) -> ObjectType{ ObjectType::SingleCharacter(self) }
}

use std::fmt::{Debug, Formatter, Error, Display};

impl Display for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.get_char())
   }
}

impl Debug for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "C({})", self.get_char())
   }
}



















