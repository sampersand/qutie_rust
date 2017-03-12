use objects::object::{Object, ObjectType};
use std::fmt::{Debug, Formatter, Error, Display};
pub trait NumberTrait : Debug + Display{}
pub struct Number<T : NumberTrait> {
   pub num_val: T
}
impl NumberTrait for i32{}

impl <T: NumberTrait> Number<T> {
   pub fn new(inp: T) -> Number<T> {
      Number{num_val: inp}
   }
}

impl <T: NumberTrait> Object for Number<T>{
   fn obj_type(&self) -> ObjectType { ObjectType::Number }
}


impl <T: NumberTrait> Display for Number<T> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.num_val)
   }
}
impl <T: NumberTrait> Debug for Number<T> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}












