use globals::IdType;
use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use objects::number::Number;
use result::{ObjError, ObjResult, BoolResult};

pub static ESCAPE_CHAR: char = '\\';

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Quote {
   Single,
   Double,
   Grave,
}

impl From<Quote> for char {
   fn from(quote: Quote) -> char {
      match quote {
         Quote::Single => '\'',
         Quote::Double => '"',
         Quote::Grave  => '`'
      }
   }
}

impl Quote {
   pub fn from_char(inp: char) -> Option<Quote> {
      if inp == char::from(Quote::Single) {
         Some(Quote::Single)
      } else if inp == char::from(Quote::Double) {
         Some(Quote::Double)
      } else if inp == char::from(Quote::Grave) {
         Some(Quote::Grave)
      } else {
         None
      }
   }
}

impl Display for Quote {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", char::from(*self))
   }
}
impl Debug for Quote {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "Q({})", self)
   }
}

pub struct Text{
   id: IdType,
   pub text_val: String,
   pub quotes: [Quote; 2],
}

impl Text{
   pub fn new(inp: String, quotes: Option<(Quote, Quote)>) -> Text {
      Text{ id: next_id!(),
            text_val: inp,
            quotes:
               if let Some(quotes) = quotes {
                  [quotes.0, quotes.1]
               } else {
                  [Quote::Single, Quote::Single]
               }
      }
   }
   pub fn to_rc(self) -> Rc<Text> {
      Rc::new(self)
   }

   pub fn to_string(&self) -> String {
      self.text_val.as_str().to_string()
   }
   fn to_repr(&self) -> String {
      self.quotes[0].to_string() + self.text_val.as_str() + self.quotes[1].to_string().as_str()
   }
   pub fn from(inp: &'static str) -> Text { // TODO: MAke this a From<str> thingy
      Text::new(inp.to_string(), None)
   }
}

macro_rules! ok_rc_text {
    ($me:expr, $text:expr) => ( Ok(Text::new($text, Some(($me.quotes[0], $me.quotes[1])).clone()).to_rc()) )
}

impl Object for Text{
   obj_functions!(OBJ_TYPE; Text);
   obj_functions!{QT_TO_BOOL; (|me: &Text| !me.text_val.is_empty())}
   obj_functions!(QT_EQL; text_val);
   // obj_functions!(QT_METHODS; text_methods);
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.to_repr().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }


   fn qt_to_text(&self, _: &mut Environment) -> Result<Rc<Text>, ObjError> {
      ok_rc_text!(self, self.text_val.clone())
   }
   fn qt_add_l(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
      let other_to_text = other.qt_to_text(env).expect("can't get text for other");
      let body = self.text_val.clone() + other_to_text.text_val.as_str();
      ok_rc_text!(self, body)
   }
   fn qt_add_r(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
      let other_to_text = other.qt_to_text(env).expect("can't get text for other");
      let body = other_to_text.text_val.clone() + self.text_val.as_str();
      ok_rc_text!(self, body)
   }
   fn qt_get_l(&self, key: ObjRc, _: &mut Environment) -> ObjResult {
      if !key.is_a(ObjType::Number) {
         panic!("Cannot index a string with: {:?}", key)
      }
      let num = cast_as!(key, Number);
      let text = 
         self.text_val
             .chars()
             .nth(num.num_val as usize)
             .expect(("invalid index: ".to_string() + num.to_string().as_str()).as_str())
             .to_string();
      ok_rc_text!(self, text)
   }
   fn qt_exec(&self, _: &mut Environment) -> ObjResult {
      assert_eq!(self.quotes[0], self.quotes[1]); // why wouldn't they be?
      match self.quotes[0] {
         Quote::Single => panic!("TODO: EXEC SINGLE QUOTES"),
         Quote::Double => panic!("TODO: EXEC DOUBLE QUOTES"),
         Quote::Grave => panic!("TODO: EXEC GRAVE QUOTES")
      }
      //   when '`' then self.class.new( `#{@text_val}`.chomp, quotes: @quotes )
      //   when "'" 
      //     result = env.parser.process( input: @text_val ).u
      //     QT_Universe.new(body: '', universe: result, parens: ['<', '>']) #to fix
      //   when '"' 
      //     result = env.parser.process( input: @text_val, universe: env.u ).u
      //     QT_Universe.new(body: '', universe: result, parens: ['<', '>']) #to fix
      //   else fail "IDK HOW TO DEAL WITH QUOTE TYPE #{@quotes[0]}"
      //   end
      // end

   }
}

impl_defaults!(DISPLAY_DEBUG; Text, 'T');













