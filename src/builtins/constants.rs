use builtins::BuiltinsType;
use objects::boolean;
use objects::obj_rc::ObjRcWrapper;
use std::rc::Rc;
use objects::symbol::Symbol;

pub fn constants() -> BuiltinsType {
   macro_rules! rc_bool {
       ($name:ident) => (rc!(boolean::$name))
   }
   map! { TYPE; BuiltinsType,
      "true"  => rc_bool!(TRUE),
      "false" => rc_bool!(FALSE),
      "null"  => rc_bool!(NULL),
      "nil"   => rc_bool!(NULL),
      "none"  => rc_bool!(NULL)
   }
}