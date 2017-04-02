use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType, ObjWrapper, OldObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;

pub struct Symbol {
   pub sym_val: String,
}

impl Symbol {
   pub fn new(inp: String) -> Symbol {
      Symbol{sym_val: inp}
   }
   pub fn to_string(&self) -> String {
      self.sym_val.to_string()
   }
   pub fn from(inp: &'static str) -> Symbol {
      Symbol::new(inp.to_string())
   }
}

impl Object for Symbol {
   impl_defaults!(OBJECT; Symbol);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; sym_val);
}

impl_defaults!(DISPLAY_DEBUG; Symbol, 'S');
