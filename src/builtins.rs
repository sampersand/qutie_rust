use objects::object::{Object, ObjType};
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
   let end_symbol = rc!(Symbol::from("sep"));
   let default_sep = rc!(Text::from(""));
   let default_end = rc!(Text::from("\n"));
   let sep = qt_try!(args.qt_get(sep_symbol, AccessType::Locals, env),
                     NoSuchKey => default_sep);
   let end = qt_try!(args.qt_get(end_symbol, AccessType::Locals, env),
                     NoSuchKey => default_end);
   let ref sep = cast_as!(*sep, Text).text_val;
   let ref end = cast_as!(*end, Text).text_val;
   for to_print in args.stack.clone(){
      print!("{}{}", to_print.qt_to_text(env).unwrap().text_val, sep)
   }
   print!("{}", end);
   ok_rc!(boolean::NULL)
}



pub fn builtins() -> GlobalsType {
   let null = rc!(boolean::NULL);
   map! { TYPE; GlobalsType,
      "true" => rc!(boolean::TRUE),
      "false" => rc!(boolean::FALSE),
      "null" => null.clone(),
      "nil" => null.clone(),
      "none" => null.clone(),
      "disp" => rc!(BuiltinFunction::new(disp_fn))
   }
}