use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn if_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let cond_num  = new_obj!(NUM, 0);
   let true_num  = new_obj!(NUM, 1);
   let false_num = new_obj!(NUM, 2);

   let cond_arg  = get_arg!(args, cond_num;  Stack, panic!("No condition!"));
   let true_arg  = get_arg!(args, true_num;  Stack, panic!("No true block!"));
   let false_arg = get_arg!(args, false_num; Stack, rc!(boolean::NULL));
   let cond = to_type!(BOOL; cond_arg, env);
   if cond {
      Ok(true_arg)
   } else {
      Ok(false_arg)
   }
}
