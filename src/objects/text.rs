use objects::object::{Object, ObjectType};
use std::fmt::{Debug, Formatter, Error, Display};

pub type TextType = String;

pub enum Quotes {
   Single,
   Double,
   Grave,
}

impl Debug for Quotes {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", match self {
         &Quotes::Single => "'",
         &Quotes::Double => "\"",
         &Quotes::Grave => "`",
      })
   }
}

pub struct Text{
   pub start_quote: Quotes,
   pub end_quote: Quotes,
   pub text_val: TextType,
}

impl Text{
   pub fn new(inp: TextType, start: Quotes, end: Quotes) -> Text {
      Text{text_val: inp, start_quote: start, end_quote: end}
   }
}

impl Object for Text{
   fn obj_type(&self) -> ObjectType { ObjectType::Text }
}


impl Display for Text{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{:?}{}{:?}", self.start_quote, self.text_val, self.end_quote)
   }
}
impl Debug for Text{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "T({:?}{}{:?})", self.start_quote, self.text_val, self.end_quote)
   }
}














