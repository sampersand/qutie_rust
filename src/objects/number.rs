use objects::object::{Object, ObjectType, QTFunctionResponse};
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

impl Object for Number{
   fn obj_type(&self) -> ObjectType { ObjectType::Number(self) }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.num_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
   fn qt_add_l(&self, other: BoxedObj) -> QTFunctionResponse{
      match other.obj_type(){
         ObjectType::Number(num_obj) => {
            let ret = Number::new(self.num_val + num_obj.num_val);
            QTFunctionResponse::Response(Box::new(ret))
         },
         _ => QTFunctionResponse::Unimplemented
      } 
   }
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












