use builtins::BuiltinsType;
use objects::object::Object;

macro_rules! rc_obj {
   (SYM; $name:expr) => ( rc!(Symbol::from($name)) );
   (TEXT; $name:expr) => ( rc!(Text::from($name)) );
   (NUM; $name:expr) => ( rc!(Number::new($name)) )
}
macro_rules! get_arg {
   ($args:expr, $env:expr, $sym:expr, $default:expr) => ( get_arg!($args, $env, $sym; Locals, $default) );
   ($args:expr, $env:expr, $sym:expr; $access_type:ident, $default:expr) => (
      match $args.qt_get($sym, AccessType::$access_type, $env){
         Ok(obj) => obj,
         Err(ObjError::NoSuchKey(_)) => $default,
         Err(err) => panic!("Error: {:?}", err)
      }
   )
}
macro_rules! to_type {
    (STRING; $inp:expr, $env:expr) => ( $inp.qt_to_text($env).unwrap().text_val.clone() );
    (BOOL; $inp:expr, $env:expr) => ( $inp.qt_to_bool($env).unwrap().bool_val );
    (NUM;  $inp:expr, $env:expr) => ( $inp.qt_to_num($env).unwrap().num_val );
}

mod disp_mod;
mod new_oper_mod;
mod literal_mod;
mod if_mod;
mod while_mod;
mod import_mod;

pub fn functions() -> BuiltinsType {
   use std::rc::Rc;
   use objects::symbol::Symbol;
   use objects::builtin_function::BuiltinFunction;
   use objects::obj_rc::ObjRcWrapper;
   macro_rules! rc_func {
       ($func:path) => (rc!(BuiltinFunction::new($func)))
   }
   map! { TYPE; BuiltinsType,
      "disp" => rc_func!(disp_mod::disp_fn),
      "new_oper" => rc_func!(new_oper_mod::new_oper_fn),
      "literal" => rc_func!(literal_mod::literal_fn),
      "if" => rc_func!(if_mod::if_fn),
      "while" => rc_func!(while_mod::while_fn),
      "import" => rc_func!(import_mod::import_fn)
   }
}
