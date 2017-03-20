use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{BoolResult, ObjError};

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;

pub struct Symbol {
   pub sym_val: String,
}

impl Symbol{
   pub fn new(inp: String) -> Symbol {
      Symbol{sym_val: inp}
   }
   pub fn to_string(&self) -> String {
      self.sym_val.to_string()
   }
}

impl Object for Symbol{
   impl_defaults!(OBJECT; Symbol);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; Symbol, sym_val);

   // fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> BoolResult {
   //    ok_rc!(Boolean::from_bool(match other.obj_type() {
   //       ObjType::Symbol(obj) => self.sym_val == obj.sym_val,
   //       _ => false
   //    }))
   // }
}

impl_defaults!(DISPLAY_DEBUG; Symbol, 'S');
