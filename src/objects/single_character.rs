use objects::object::{Object, ObjectType};

type SourceType = char;

#[derive(Eq, PartialEq)]
pub struct SingleCharacter {
   pub source_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{source_val: inp}
   }
}

impl Object for SingleCharacter{
   fn obj_type(&self) -> ObjectType{ ObjectType::SingleCharacter(self) }
}

use std::fmt::{Debug, Formatter, Error, Display};

impl Display for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.source_val)
   }
}

impl Debug for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "C({})", self)
   }
}



















