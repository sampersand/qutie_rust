use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn literal_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let mut oper_acc = String::new();
   for arg in args.stack.iter() {
      oper_acc += arg.qt_to_text(env).unwrap().text_val.as_str();
   }
   let oper_acc_dup = oper_acc.clone();
   let oper_sym = rc!(Symbol::new(oper_acc));
   match env.universe.get(oper_sym, AccessType::NonStack) {
      Ok(obj) => Ok(obj),
      Err(ObjError::NoSuchKey(_)) => panic!("No such literal found: {:?}", oper_acc_dup),
      Err(err) => panic!("TODO: Handle this error: {:?}", err)
   }
}