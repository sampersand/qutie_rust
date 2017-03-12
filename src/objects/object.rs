use std::fmt::{Debug, Display};
use objects::single_character;
use objects::operator;

#[derive(Debug)]
pub enum ObjectType<'a> {
   Universe,
   Number,
   SingleCharacter(&'a single_character::SingleCharacter),
   Symbol,
   Text,
   Boolean,
   Operator(&'a operator::Operator)
}
pub trait Object : Debug + Display {
   fn obj_type(&self) -> ObjectType;
   // fn as_type(&self, to_type: T) -> T {
   //    match self.obj_type() {
   //       e @ to_type => e,
   //       e @ _ => panic!("Bad type: {:?}", e)
   //    }
   // }
}