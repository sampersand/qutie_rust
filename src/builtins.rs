use objects::object::{Object, ObjType};
use objects::obj_rc::ObjRcWrapper;
use objects::universe::{Universe, GlobalsType, AccessType};
use objects::symbol::Symbol;
use objects::builtin_function::BuiltinFunction;
use objects::operator::{Operator, OperFunc};
use env::Environment;
use objects::boolean;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjResult, ObjError};

fn disp_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let sep_symbol = rc!(Symbol::from("sep"));
   let end_symbol = rc!(Symbol::from("sep"));
   let default_sep = rc!(Text::from(""));
   let default_end = rc!(Text::from("\n"));
   let sep = qt_try!(args.qt_get(sep_symbol, AccessType::Locals, env), NoSuchKey => default_sep);
   let end = qt_try!(args.qt_get(end_symbol, AccessType::Locals, env), NoSuchKey => default_end);
   let ref sep = cast_as!(*sep, Text).text_val;
   let ref end = cast_as!(*end, Text).text_val;
   for to_print in args.stack.clone(){
      print!("{}{}", to_print.qt_to_text(env).unwrap().text_val, sep)
   }
   print!("{}", end);
   ok_rc!(boolean::NULL)
}

fn define_oper(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let sigil_sym    = rc!(Symbol::from("sigil"));
   let rhs_sym      = rc!(Symbol::from("rhs"));
   let lhs_sym      = rc!(Symbol::from("lhs"));
   let priority_sym = rc!(Symbol::from("priority"));
   let func_sym     = rc!(Symbol::from("func"));
   let sigil    = qt_try!(args.qt_get(sigil_sym, AccessType::Locals, env), NoSuchKey => panic!("Can't find sigil"));
   let rhs      = qt_try!(args.qt_get(rhs_sym, AccessType::Locals, env), NoSuchKey => panic!("Can't find rhs"));
   let lhs      = qt_try!(args.qt_get(lhs_sym, AccessType::Locals, env), NoSuchKey => panic!("Can't find lhs"));
   let priority = qt_try!(args.qt_get(priority_sym, AccessType::Locals, env), NoSuchKey => panic!("Can't find priority"));
   let func     = qt_try!(args.qt_get(func_sym, AccessType::Locals, env), NoSuchKey => panic!("Can't find func"));
   
   let sigil = sigil.qt_to_text(env).unwrap().text_val.clone();
   let lhs = lhs.qt_to_bool(env).unwrap().bool_val;
   let rhs = rhs.qt_to_bool(env).unwrap().bool_val;
   let priority = priority.qt_to_num(env).unwrap().num_val as u32;
   let func = OperFunc::Callable(func);
   let oper = Operator::new(rc!(sigil.clone()), lhs, rhs, priority, func);
   env.universe.set(rc!(Symbol::new(sigil)), rc!(oper), AccessType::Locals)
}

fn if_fn()


pub fn builtins() -> GlobalsType {
   let null = rc!(boolean::NULL);
   map! { TYPE; GlobalsType,
      "true" => rc!(boolean::TRUE),
      "false" => rc!(boolean::FALSE),
      "null" => null.clone(),
      "nil" => null.clone(),
      "none" => null.clone(),
      "disp" => rc!(BuiltinFunction::new(disp_fn)),
      "define_oper" => rc!(BuiltinFunction::new(define_oper))
   }
}






