use builtins::BuiltinsType;

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
mod if_fn;
mod types;
mod while_fn;
mod import;
mod syscall;
mod stop;

pub fn functions() -> BuiltinsType {
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
      "if_fn" => rc_func!(if_fn::if_fn),
      "while_fn" => rc_func!(while_fn::while_fn),
      "import" => rc_func!(import::import_fn),
      "syscall" => rc_func!(syscall::syscall_fn),
      "text" => rc_func!(types::text_fn),
      "bool" => rc_func!(types::bool_fn),
      "num" => rc_func!(types::num_fn),
      "stop" => rc_func!(stop::stop_fn)
   }
}







   







