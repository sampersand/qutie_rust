use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use env::Environment;
use std::rc::Rc;
use objects::universe::AccessType;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use result::{ObjError, ObjResult};

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
      self.text_val.as_str().to_string()
      // self.quotes[0].to_string() + self.text_val.as_str() + self.quotes[1].to_string().as_str()
   }
   pub fn from(inp: &'static str) -> Text {
      Text::new(inp.to_string(), [Quote::Single, Quote::Single])
   }
}

impl Object for Text{
   impl_defaults!(OBJECT; Text);
   obj_functions!{QT_TO_BOOL; (|me: &Text| !me.text_val.is_empty())}
   obj_functions!(QT_EQL; Text, text_val);


   fn qt_to_text(&self, _: &mut Environment) -> Result<Rc<Text>, ObjError> {
      ok_rc!(Text::new(self.text_val.clone(), self.quotes.clone()))
   }
   fn qt_add_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      let other_to_text = other.qt_to_text(env).unwrap();
      let body = self.text_val.clone() + other_to_text.text_val.as_str();
      ok_rc!(Text::new(body, self.quotes.clone()))
   }
   fn qt_add_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      let other_to_text = other.qt_to_text(env).unwrap();
      let body = other_to_text.text_val.clone() + self.text_val.as_str();
      ok_rc!(Text::new(body, self.quotes.clone()))
   }
   fn qt_get(&self, key: ObjRc, a_type: AccessType, env: &mut Environment) -> ObjResult {
      if a_type != AccessType::All {
         panic!("Bad access type {:?}", a_type)
      }
      if let ObjType::Number(num) = key.obj_type() {
         let text = self.text_val
                        .chars()
                        .nth(num.num_val as usize)
                        .expect(("invalid index: ".to_string() + num.to_string().as_str()).as_str())
                        .to_string();
         ok_rc!(Text::new(text, self.quotes.clone()))
      } else {
         panic!("Cannot index a string with: {:?}", key)
      }
   }
}

impl_defaults!(DISPLAY_DEBUG; Text, 'T');










