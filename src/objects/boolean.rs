use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;

pub enum Boolean {
   True,
   False,
   Null
}

impl Boolean {
   pub fn to_string(&self) -> String {
      self.to_bool().to_string()
   }

   pub fn to_bool(&self) -> bool{
      match *self {
         Boolean::True => true,
         Boolean::False => false,
         Boolean::Null => false,
      }
   }
   pub fn from_bool(inp: bool) -> Boolean {
      match inp {
         true => Boolean::True,
         false => Boolean::False
      }
   }
}

impl Object for Boolean {
   impl_defaults!(OBJECT; Boolean);
}

impl_defaults!(DISPLAY_DEBUG; Boolean, 'B');












