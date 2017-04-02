use builtins::BuiltinsType;
use objects::object::Object;

macro_rules! get_arg {
   ($args:expr, $sym:expr, $default:expr) => ( get_arg!($args, $sym; Locals, $default) );
   ($args:expr, $sym:expr; $access_type:ident, $default:expr) => (
      match $args.get($sym, AccessType::$access_type){
         Ok(obj) => obj,
         Err(ObjError::NoSuchKey(_)) => $default,
         Err(err) => panic!("Error: {:?}", err)
      }
   )
}

mod disp_mod;
mod new_oper_mod;
mod literal_mod;
mod if_mod;
mod while_mod;
mod import_mod;
mod syscall_mod;
mod method_mod;

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
      "import" => rc_func!(import_mod::import_fn),
      "syscall" => rc_func!(syscall_mod::syscall_fn),
      "method" => rc_func!(method_mod::method_fn)
   }
}







   