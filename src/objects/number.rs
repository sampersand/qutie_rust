use objects::object::{Object, ObjectType, FunctionResponse, FunctionError};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;
use objects::boxed_obj::BoxedObj;

pub type NumberType = i32;

pub struct Number {
   pub num_val: NumberType
}


impl Number {
   pub fn new(inp: NumberType) -> Number {
      Number{num_val: inp}
   }
}

macro_rules! num_oper_func {
   ( $name_l:ident, $name_r:ident, $oper:tt ) => {
      fn $name_l(&self, other: &BoxedObj) -> FunctionResponse{
         match other.qt_to_num() {
            Ok(num_obj) => Ok(Box::new(Number::new(self.num_val $oper num_obj.num_val ))),
            Err(FunctionError::NotImplemented) => Err(FunctionError::NoResponse)
         }
      }
   }
}
use std;
impl Object for Number{
   fn obj_type(&self) -> ObjectType { ObjectType::Number(self) }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.num_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
   num_oper_func!(qt_add_l, qt_add_r, +);
}


impl Display for Number {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.num_val)
   }
}
impl Debug for Number {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}












