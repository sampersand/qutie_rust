use env::Environment;
use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use result::BoolResult;
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
   impl_defaults!{OBJECT; Symbol}

   fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> BoolResult {
      ok_rc!(Boolean::from_bool(match other.obj_type() {
         ObjType::Symbol(obj) => self.sym_val == obj.sym_val,
         _ => false
      }))
   }
}

impl_defaults!{DISPLAY_DEBUG; Symbol, 'S'}
