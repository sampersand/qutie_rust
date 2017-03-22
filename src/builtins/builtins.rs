use objects::object::{Object, ObjType};
use objects::obj_rc::ObjRcWrapper;
use objects::universe::{Universe, GlobalsType, AccessType};
use objects::symbol::Symbol;
use objects::builtin_function::BuiltinFunction;
use objects::operator::{Operator, OperFunc};
use env::Environment;
use objects::boolean;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjResult, ObjError};


fn if_fn()


pub fn builtins() -> GlobalsType {
   let null = rc!(boolean::NULL);
   map! { TYPE; GlobalsType,
      "true" => rc!(boolean::TRUE),
      "false" => rc!(boolean::FALSE),
      "null" => null.clone(),
      "nil" => null.clone(),
      "none" => null.clone(),
      "disp" => rc!(BuiltinFunction::new(disp_fn)),
      "define_oper" => rc!(BuiltinFunction::new(define_oper))
   }
}






