use env::Environment;

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::text::Text;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use result::{ObjError, BoolResult};

#[derive(Clone)]
pub struct Boolean {
   pub bool_val: bool
}
pub const TRUE:  Boolean = Boolean{ bool_val: true };
pub const FALSE: Boolean = Boolean{ bool_val: false };
pub const NULL:  Boolean = Boolean{ bool_val: false };

impl Boolean {
   pub fn to_string(&self) -> String {
      if self as *const Boolean == &NULL as *const Boolean {
         "null".to_string()
      }
      else {
         self.bool_val.to_string()
      }
   }

   pub fn from_bool(inp: bool) -> Boolean {
      match inp {
         true => TRUE,
         false => FALSE
      }
   }
}

impl Object for Boolean {
   impl_defaults!(OBJECT; Boolean);
   obj_functions!(QT_TO_BOOL; (|me: &Boolean| me.bool_val));
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; Boolean, bool_val);
}

impl_defaults!(DISPLAY_DEBUG; Boolean, 'B');












