use std::rc::Rc;
use std::process;
use objects::universe::{Universe, AccessType};
use objects::number::Number;
use objects::object::ObjType;

use builtins::functions::disp::disp_fn;
use env::Environment;
use result::{ObjResult, ObjError};

#[allow(unused_must_use)]
pub fn stop_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let reason_pos  = new_obj!(NUM, 0);
   let reason  = get_arg!(args, reason_pos; Stack, panic!("No reason!"));
   process::exit(
      if reason.is_a(ObjType::Number) {
         to_type!(NUM; reason, env)
      } else {
         disp_fn(args, env);
         0
      }
   );
}
