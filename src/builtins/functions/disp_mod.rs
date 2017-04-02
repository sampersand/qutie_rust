use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn disp_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sep_sym = new_obj!(SYM_STATIC, "sep");
   let end_sym = new_obj!(SYM_STATIC, "end");
   let sep_def = new_obj!(TEXT_STATIC, "");
   let end_def = new_obj!(TEXT_STATIC, "\n");

   /* attempt to find args */
   let sep_arg = get_arg!(args, sep_sym, sep_def);
   let end_arg = get_arg!(args, end_sym, end_def);

   /* cast args to right type */
   let ref sep = to_type!(STRING; sep_arg, env);
   let ref end = to_type!(STRING; end_arg, env);

   /* print it out */
   if let Some(obj) = args.stack.get(0) {
      print!("{}", obj);
      for to_print in &args.stack[1..args.stack.len()] {
         print!("{}{}", sep, to_type!(STRING; to_print, env))
      }
      print!("{}", end);
   }

   /* return */
   ok_rc!(boolean::NULL)
}





