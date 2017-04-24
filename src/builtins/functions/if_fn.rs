use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::number::Number;
use objects::boolean::{Boolean, BoolType};
use objects::object::{ObjType, ObjWrapper};
use objects::universe::ParenType;

use env::Environment;
use result::{ObjResult, ObjError};

#[allow(unused_must_use)]
pub fn if_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let cond_num  = new_obj!(NUM, 0);
   let true_num  = new_obj!(NUM, 1);
   let false_num = new_obj!(NUM, 2);

   let cond_arg  = get_arg!(args, cond_num;  Stack, panic!("No condition!"));
   let true_arg  = get_arg!(args, true_num;  Stack, panic!("No true block!"));
   let false_arg = get_arg!(args, false_num; Stack, new_obj!(BOOL_STATIC, Null));
   let cond = to_type!(BOOL; cond_arg, env);
   let ret = 
      if cond {
         true_arg
      } else {
         false_arg
      };
   if ret.is_a(ObjType::Universe) && cast_as!(CL; ret, Universe).parens.0 == ParenType::Curly {
      cast_as!(ret, Universe).exec_all(env);
      if let Some(obj) = env.universe.stack.pop(){
         Ok(obj)
      } else {
         Ok(new_obj!(BOOL_STATIC, Null))
      }
   } else {
      Ok(ret)
   }
}
