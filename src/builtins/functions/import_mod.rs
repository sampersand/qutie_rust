use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};


pub fn import_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let name_num  = rc_obj!(NUM; 0);
   let name_arg = get_arg!(args, env, name_num; Stack, panic!("No body block!"));
   
   panic!();
}