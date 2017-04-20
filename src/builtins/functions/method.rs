use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::number::Number;
use objects::boolean;
use objects::object::{Object, ObjType, ObjWrapper};

use env::Environment;
use result::{ObjResult, ObjError};

pub fn method_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let obj_num  = new_obj!(NUM, 0);
   let method_num  = new_obj!(NUM, 1);
   let obj_arg = get_arg!(args, obj_num; Stack, panic!("No object!"));
   let method_arg = get_arg!(args, method_num; Stack, panic!("No method!"));

   let obj = obj_arg;
   let method = to_type!(STRING; method_arg, env);
   match obj.qt_method(method.as_str(), env) {
      Ok(obj) => Ok(obj),
      Err(ObjError::NoSuchKey(key)) => panic!("No method with name {:?} found", to_type!(STRING; key, env)),
      Err(err) => panic!("bad error: {:?}", err)
   }
}







