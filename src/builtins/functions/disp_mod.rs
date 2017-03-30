use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

pub fn disp_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sep_sym = rc_obj!(SYM; "sep");
   let end_sym = rc_obj!(SYM; "end");
   let sep_def = rc_obj!(TEXT; "");
   let end_def = rc_obj!(TEXT; "\n");

   /* attempt to find args */
   let sep_arg = get_arg!(args, env, sep_sym, sep_def);
   let end_arg = get_arg!(args, env, end_sym, end_def);

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





