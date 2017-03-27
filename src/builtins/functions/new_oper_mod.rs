use std::rc::Rc;
use objects::operator::{Operator, OperFunc};
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::number::Number;
use objects::text::Text;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn new_oper_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sigil_num = rc_obj!(NUM; 0);
   let lhs_num   = rc_obj!(NUM; 1);
   let rhs_num   = rc_obj!(NUM; 2);
   let prior_num = rc_obj!(NUM; 3);
   let func_num  = rc_obj!(NUM; 4);

   /* attempt to find args */
   let sigil_arg = get_arg!(args, env, sigil_num; Stack, panic!("Can't find sigil"));
   let lhs_arg   = get_arg!(args, env, lhs_num; Stack, panic!("Can't find lhs"));
   let rhs_arg   = get_arg!(args, env, rhs_num; Stack, panic!("Can't find rhs"));
   let prior_arg = get_arg!(args, env, prior_num; Stack, panic!("Can't find priority"));
   let func_arg  = get_arg!(args, env, func_num; Stack, panic!("Can't find func"));
   
   /* convert to types required by Operator::new */
   let sigil    = to_type!(STRING; sigil_arg, env);
   let lhs      = to_type!(BOOL; lhs_arg, env);
   let rhs      = to_type!(BOOL; rhs_arg, env);
   let priority = to_type!(NUM;  prior_arg, env) as u32;
   let func = OperFunc::Callable(func_arg);

   /* Create oper and assign it */
   let oper = Operator::new(sigil.clone(), lhs, rhs, priority, func);
   // Ok(rc!(oper))
   env.universe.set(rc!(Symbol::new(sigil)), rc!(oper), AccessType::Locals)
}
