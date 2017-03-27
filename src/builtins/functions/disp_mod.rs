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
   let ref sep = to_type!(TEXT; sep_arg, env);
   let ref end = to_type!(TEXT; end_arg, env);

   /* print it out */
   for to_print in args.stack.clone(){
      print!("{}{}", to_type!(TEXT; to_print, env), sep)
   }
   print!("{}", end);

   /* return */
   ok_rc!(boolean::NULL)
}
