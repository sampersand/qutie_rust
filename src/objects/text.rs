use objects::object::{Object, ObjectType};
use objects::single_character::SingleCharacter;
use std::fmt::{Debug, Formatter, Error, Display};

pub type TextType = String;
pub static ESCAPE: SingleCharacter = SingleCharacter{source_val: '\\'};

#[derive(PartialEq, Eq)]
pub enum Quote {
   Single,
   Double,
   Grave,
}
impl Quote {
   pub fn to_char(&self) -> char {
      match *self {
         Quote::Single => '\'',
         Quote::Double => '"',
         Quote::Grave  => ','
      }
   }
   pub fn from_single_char(single_char: &SingleCharacter) -> Option<Quote> {
      let source_val = single_char.source_val;
      if source_val == Quote::Single.to_char() { Some(Quote::Single) }
      else if source_val == Quote::Double.to_char() { Some(Quote::Double) }
      else if source_val == Quote::Grave.to_char() { Some(Quote::Grave) }
      else { None }
   }
}

impl Display for Quote {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.to_char())
   }
}
impl Debug for Quote {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "Q({})", self)
   }
}

pub struct Text{
   pub start_quote: Quote,
   pub end_quote: Quote,
   pub text_val: TextType,
}

impl Text{
   pub fn new(inp: TextType, start: Quote, end: Quote) -> Text {
      Text{text_val: inp, start_quote: start, end_quote: end}
   }
}

impl Object for Text{
   fn obj_type(&self) -> ObjectType { ObjectType::Text }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.text_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }

      ret
   }
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














