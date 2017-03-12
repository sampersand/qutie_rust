use objects::object::{Object, ObjectType};
use std::fmt::{Debug, Formatter, Error, Display};


pub struct Symbol {
   pub sym_val: String,
}

impl Symbol{
   pub fn new(inp: String) -> Symbol {
      Symbol{sym_val: inp}
   }
}

impl Object for Symbol{
   fn obj_type(&self) -> ObjectType { ObjectType::Symbol }
}


impl Display for Symbol{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.sym_val)
   }
}
impl Debug for Symbol{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "S({})", self)
   }
}