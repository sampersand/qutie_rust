use builtins::BuiltinsType;
use objects::boolean::{Boolean, BoolType};
use objects::number::Number;
use objects::obj_rc::ObjRcWrapper;
use std::rc::Rc;
use objects::symbol::Symbol;
pub fn constants() -> BuiltinsType {
   map! { TYPE; BuiltinsType,
      "true"  => new_obj!(BOOL_STATIC, True),
      "false" => new_obj!(BOOL_STATIC, False),
      "null"  => new_obj!(BOOL_STATIC, Null),
      "nil"   => new_obj!(BOOL_STATIC, Null),
      "none"  => new_obj!(BOOL_STATIC, Null),
      "NEG_1" => new_obj!(NUM, -1)
   }
}