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

mod disp;
mod new_oper;
mod literal;
mod if_;
mod types;
mod while_;
mod import;
mod syscall;
mod method;

pub fn functions() -> BuiltinsType {
   use std::rc::Rc;
   use objects::symbol::Symbol;
   use objects::builtin_function::BuiltinFunction;
   use objects::obj_rc::ObjRcWrapper;
   macro_rules! rc_func {
       ($func:path) => ( BuiltinFunction::new($func).to_rc() ) 
   }
   map! { TYPE; BuiltinsType,
      "disp" => rc_func!(disp::disp_fn),
      "new_oper" => rc_func!(new_oper::new_oper_fn),
      "literal" => rc_func!(literal::literal_fn),
      "if" => rc_func!(if_::if_fn),
      "while" => rc_func!(while_::while_fn),
      "import" => rc_func!(import::import_fn),
      "syscall" => rc_func!(syscall::syscall_fn),
      "method" => rc_func!(method::method_fn),
      "text" => rc_func!(types::text_fn),
      "bool" => rc_func!(types::bool_fn),
      "num" => rc_func!(types::num_fn)
   }
}







   







