use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use env::Environment;
use std::rc::Rc;
use objects::boolean::Boolean;
use result::{BoolResult, ObjError};

pub type TextType = String;
pub static ESCAPE_CHAR: char = '\\';

#[derive(PartialEq, Eq, Clone, Copy)]
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
         Quote::Grave  => '`'
      }
   }
   pub fn from_char(inp: char) -> Option<Quote> {
      if inp == Quote::Single.to_char() {
         Some(Quote::Single)
      } else if inp == Quote::Double.to_char() {
         Some(Quote::Double)
      } else if inp == Quote::Grave.to_char() {
         Some(Quote::Grave)
      } else {
         None
      }
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
   pub text_val: TextType,
   pub quotes: [Quote; 2],
}

impl Text{
   pub fn new(inp: TextType, quotes: [Quote; 2]) -> Text {
      Text {text_val: inp, quotes: quotes }
   }
   pub fn to_string(&self) -> String {
      self.quotes[0].to_string() + self.text_val.as_str() + self.quotes[1].to_string().as_str()
   }
}

impl Object for Text{
   impl_defaults!(OBJECT; Text);
   obj_functions!{QT_TO_BOOL; (|me: &Text| !me.text_val.is_empty())}
   fn qt_to_text(&self, _: &mut Environment) -> Result<Rc<Text>, ObjError> {
      ok_rc!(Text::new(self.text_val.clone(), self.quotes.clone()))
   }
}

impl_defaults!(DISPLAY_DEBUG; Text, 'T');










