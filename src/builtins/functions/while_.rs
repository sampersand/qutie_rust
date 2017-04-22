use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::number::Number;
use objects::boolean::{Boolean, BoolType};
use objects::object::{Object, ObjWrapper};

use env::Environment;
use result::{ObjResult, ObjError};

pub fn while_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let cond_num  = new_obj!(NUM, 0);
   let body_num  = new_obj!(NUM, 1);
   let cond_arg = get_arg!(args, cond_num; Stack, panic!("No condition!"));
   let body_arg = get_arg!(args, body_num; Stack, panic!("No body block!"));

   let cond = cast_as!(cond_arg, Universe);
   let body = cast_as!(body_arg, Universe);
   let mut ret: ObjResult = Ok(new_obj!(BOOL_STATIC, Null));
   loop {
      match cond.clone().qt_exec(env){
         Ok(obj) =>
            match cast_as!(obj, Universe).get(new_obj!(NUM, 0), AccessType::Stack) {
               Ok(obj) =>
                  if to_type!(BOOL; obj, env) {
                     ret = body.qt_exec(env);
                  } else {
                     break
                  },
               Err(err) => panic!("While condition returned error: {:?}", err)
            },
         Err(err) => panic!("Howto error?: {:?}", err) 
      }
   }
   ret
}
