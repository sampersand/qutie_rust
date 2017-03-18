use objects::object::{Object, ObjType};

type SourceType = char;

#[derive(Eq, PartialEq, Clone)]
pub struct SingleCharacter {
   pub char_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{char_val: inp}
   }
}

impl Object for SingleCharacter{
   fn obj_type(&self) -> ObjType{ ObjType::SingleCharacter(self) }
   fn source(&self) -> Vec<SingleCharacter> { vec![self.clone()] }
}

use std::fmt::{Debug, Formatter, Error, Display};

impl Display for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.char_val)
   }
}

impl Debug for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "C({})", self)
   }
}



















