use builtins::BuiltinsType;
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

macro_rules! rc_sym {
   ($name:expr) => ( rc!(Symbol::from($name)) );
   (TEXT; $name:expr) => ( rc!(Text::from($name)) )
}
macro_rules! get_arg {
   ($args:expr, $env:expr, $sym:expr, $default:expr) => (
      qt_try!($args.qt_get($sym, AccessType::Locals, $env), NoSuchKey => $default)
   )
}
macro_rules! to_type {
    (TEXT; $inp:expr, $env:expr) => ( $inp.qt_to_text($env).unwrap().text_val.clone() );
    (BOOL; $inp:expr, $env:expr) => ( $inp.qt_to_bool($env).unwrap().bool_val );
    (NUM;  $inp:expr, $env:expr) => ( $inp.qt_to_num($env).unwrap().num_val );
}
fn disp_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sep_sym = rc_sym!("sep");
   let end_sym = rc_sym!("end");
   let sep_def = rc_sym!(TEXT; "");
   let end_def = rc_sym!(TEXT; "\n");

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

fn define_oper(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sigil_sym    = rc_sym!("sigil");
   let rhs_sym      = rc_sym!("rhs");
   let lhs_sym      = rc_sym!("lhs");
   let priority_sym = rc_sym!("priority");
   let func_sym     = rc_sym!("func");

   /* attempt to find args */
   let sigil_arg    = get_arg!(args, env, sigil_sym,    panic!("Can't find sigil"));
   let rhs_arg      = get_arg!(args, env, rhs_sym,      panic!("Can't find rhs"));
   let lhs_arg      = get_arg!(args, env, lhs_sym,      panic!("Can't find lhs"));
   let priority_arg = get_arg!(args, env, priority_sym, panic!("Can't find priority"));
   let func_arg     = get_arg!(args, env, func_sym,     panic!("Can't find func"));
   
   /* convert to types required by Operator::new */
   let sigil    = to_type!(TEXT; sigil_arg, env);
   let lhs      = to_type!(BOOL; lhs_arg, env);
   let rhs      = to_type!(BOOL; rhs_arg, env);
   let priority = to_type!(NUM;  priority_arg, env) as u32;
   let func = OperFunc::Callable(func_arg);

   /* Create oper and assign it */
   let oper = Operator::new(rc!(sigil.clone()), lhs, rhs, priority, func);
   env.universe.set(sigil_arg, rc!(oper), AccessType::Locals)
}



pub fn functions() -> BuiltinsType {
   macro_rules! name {
       () => ()
   }
   map! { TYPE; BuiltinsType,
      "disp" => rc!(BuiltinFunction::new(disp_fn)),
      "define_oper" => rc!(BuiltinFunction::new(define_oper)),
      "if" => 
   }
}






