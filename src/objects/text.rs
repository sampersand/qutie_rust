use objects::object::{Object, ObjectType};
use objects::single_character::SingleCharacter;
use std::fmt::{Debug, Formatter, Error, Display};

pub type TextType = String;
pub static ESCAPE: SingleCharacter = SingleCharacter{source_val: '\\'};

#[derive(PartialEq, Eq)]
pub enum Quotes {
   Single,
   Double,
   Grave,
}
impl Quotes {
   pub fn get_quote(inp: &SingleCharacter) -> Option<Quotes> {
      match inp.source_val {
         '\'' => Some(Quotes::Single),
         '"'  => Some(Quotes::Double),
         '`'  => Some(Quotes::Grave),
            _ => None
      }
   }
}

impl Display for Quotes {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", match self {
         &Quotes::Single => "'",
         &Quotes::Double => "\"",
         &Quotes::Grave => "`",
      })
   }
}
impl Debug for Quotes {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "Q({})", self)
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
      write!(f, "{}{}{}", self.start_quote, self.text_val, self.end_quote)
   }
}
impl Debug for Text{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "T({})", self)
   }
}














