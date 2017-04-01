use objects::text::Text;
use objects::object::Object;
use env::Environment;
use result::ObjResult;
use objects::universe::Universe;
use std::rc::Rc;
use objects::symbol::Symbol;
use objects::builtin_method::BuiltinMethod;

use objects::obj_rc::ObjRc;

fn length(obj: &Object, args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   assert_eq!(args.stack.len(), 0);
   assert_eq!(args.locals.len(), 0);
   panic!()

}

pub fn get_method(text: &Text, meth: &str, env: &mut Environment) -> ObjResult {

   match meth {
      "len" | "length" | "size" => Ok(rc_meth!(text, length)),
      _ => no_method!(meth)
   }
}