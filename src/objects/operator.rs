use objects::object::{Object, ObjectType};
use objects::universe::Universe;
use std::fmt::{Debug, Formatter, Error, Display};

#[derive(Clone)]
pub enum Operator {
   Equals
}

impl Operator {
   pub fn get_value(&self) -> &str {
      match self {
         &Operator::Equals => "="
      }
   }
   pub fn operands(&self) -> Vec<usize> {
      match self {
         &Operator::Equals => vec![1, 1]
      }
   }
   pub fn priority(&self) -> u32 {
      match self {
         &Operator::Equals => 35
      }
   }
}
impl Object for Operator {
   fn obj_type(&self) -> ObjectType { ObjectType::Operator(self) }
}

impl Display for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.get_value())
   }
}
impl Debug for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "O({})", self)
   }
}