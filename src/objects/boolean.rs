use env::Environment;

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::text::Text;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use result::{ObjError, ObjResult};

pub enum BoolType {
   True, False, Null
}

#[derive(Clone)]
pub struct Boolean {
   pub bool_val: bool,
   is_null: bool
}
use std::sync::Arc;

const TRUE:  Boolean = Boolean{ bool_val: true, is_null: false };
const FALSE: Boolean = Boolean{ bool_val: false, is_null: false };
const NULL:  Boolean = Boolean{ bool_val: false, is_null: true };

impl Boolean {
   pub fn to_string(&self) -> String {
      if self.is_null {
         "null".to_string()
      } else {
         self.bool_val.to_string()
      }
   }
   pub fn to_rc(self) -> Rc<Boolean> {
      Rc::new(self)
   }
}

impl From<BoolType> for Rc<Boolean> {
   fn from(inp: BoolType) -> Rc<Boolean> {
      match inp {
         BoolType::True => rc!(TRUE),
         BoolType::False => rc!(FALSE),
         BoolType::Null => rc!(NULL)
      }
   }
}
impl From<bool> for Boolean {
   fn from(inp: bool) -> Boolean {
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
   obj_functions!(QT_EQL; bool_val);
}

impl_defaults!(DISPLAY_DEBUG; Boolean, 'B');












