use objects::object::Object;
use objects::obj_rc::ObjRcWrapper;
use objects::universe::{Universe, GlobalsType, AccessType};
use objects::symbol::Symbol;
use objects::builtin_function::BuiltinFunction;
use env::Environment;
use objects::boolean;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjResult, ObjError};

fn disp_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult{
   let sep_symbol = rc!(Symbol::from("sep"));
   let endl_symbol = rc!(Symbol::from("sep"));
   let default_sep = rc!(Text::from(", "));
   let default_endl = rc!(Text::from("\n"));
   let sep  = qt_try!(args.qt_get(sep_symbol, AccessType::Locals, env),
                      NoSuchKey => default_sep);
   let endl = qt_try!(args.qt_get(endl_symbol, AccessType::Locals, env),
                      NoSuchKey => default_endl);
   println!("{:?}", args); /* TODO: THIS */
   ok_rc!(boolean::NULL)
}



pub fn builtins() -> GlobalsType {
   let null = rc!(boolean::NULL);
   let builtins: GlobalsType = map! { TYPE; GlobalsType,
      "true" => rc!(boolean::TRUE),
      "false" => rc!(boolean::FALSE),
      "null" => null.clone(),
      "nil" => null.clone(),
      "none" => null.clone(),
      "disp" => rc!(BuiltinFunction::new(disp_fn))
   };
   builtins
}