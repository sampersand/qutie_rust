use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::number::Number;
use objects::boolean;
use objects::object::{Object, ObjType};

use env::Environment;
use result::{ObjResult, ObjError};

pub fn while_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let cond_num  = rc_obj!(NUM; 0);
   let body_num  = rc_obj!(NUM; 1);
   println!("{:?}", "A");
   let cond_arg = get_arg!(args, env, cond_num; Stack, panic!("No condition!"));
   println!("{:?}", "B");
   let body_arg = get_arg!(args, env, body_num; Stack, panic!("No body block!"));
   println!("{:?}", "C");

   let cond = cast_as!(cond_arg, Universe);
   let body = cast_as!(body_arg, Universe);
   println!("{:?} : {:?}", cond, body);
   loop {
      match cond.stream_clone().qt_exec(env){
         Ok(obj) => match obj.qt_get(rc_obj!(NUM; 0), AccessType::Stack, env) {
            Ok(obj) => if to_type!(BOOL; obj, env) {
                          cast_as!(body, Universe).stream_clone().qt_exec(env);
                       } else {
                          break
                       },
            Err(err) => panic!("While condition returned error: {:?}", err)
         },
         Err(err) => panic!("Howto error?: {:?}", err) 
      }
   }
   ok_rc!(boolean::NULL)
}
