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
}

impl Object for Symbol{
   fn obj_type(&self) -> ObjType { ObjType::Symbol(&self) }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.sym_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
   fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> BoolResult {
      ok_rc!(Boolean::from_bool(match other.obj_type() {
         ObjType::Symbol(obj) => self.sym_val == obj.sym_val,
         _ => false
      }))
   }
}

display_debug!(Symbol, 'S', sym_val);
