use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::number::Number;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn num_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let arg_pos = new_obj!(NUM, 0);
   let arg = get_arg!(args, arg_pos; Stack, panic!("No argument found"));
   match arg.qt_to_num(env){ // because implicit casting
      Ok(obj) => Ok(obj), 
      Err(err) => Err(err)
   }
}
pub fn text_fn(_: Rc<Universe>, _: &mut Environment) -> ObjResult {
   panic!()
}
pub fn bool_fn(_: Rc<Universe>, _: &mut Environment) -> ObjResult {
   panic!()
}