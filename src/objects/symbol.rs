use objects::object::{Object, ObjType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;

use objects::obj_rc::ObjRc;
use result::BoolResult;
use objects::boolean::Boolean;

pub struct Symbol {
   pub sym_val: String,
}

impl Symbol{
   pub fn new(inp: String) -> Symbol {
      Symbol{sym_val: inp}
   }
}

impl Object for Symbol{
   fn obj_type(&self) -> ObjType { ObjType::Symbol(&self) }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.sym_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
   fn qt_eql_l(&self, other: &ObjRc) -> BoolResult {
      Ok(Box::new(Boolean::from_bool(match other.obj_type() {
         ObjType::Symbol(obj) => self.sym_val == obj.sym_val,
         _ => false
      })))
   }
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