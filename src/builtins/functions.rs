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
use objects::number::Number;
use std::rc::Rc;
use result::{ObjResult, ObjError};

macro_rules! rc_obj {
   (SYM; $name:expr) => ( rc!(Symbol::from($name)) );
   (TEXT; $name:expr) => ( rc!(Text::from($name)) );
   (NUM; $name:expr) => ( rc!(Number::new($name)) )
}
macro_rules! get_arg {
   ($args:expr, $env:expr, $sym:expr, $default:expr) => ( get_arg!($args, $env, $sym; Locals, $default) );
   ($args:expr, $env:expr, $sym:expr; $access_type:ident, $default:expr) => (
      qt_try!($args.qt_get($sym, AccessType::$access_type, $env), NoSuchKey => $default)
   )
}
macro_rules! to_type {
    (TEXT; $inp:expr, $env:expr) => ( $inp.qt_to_text($env).unwrap().text_val.clone() );
    (BOOL; $inp:expr, $env:expr) => ( $inp.qt_to_bool($env).unwrap().bool_val );
    (NUM;  $inp:expr, $env:expr) => ( $inp.qt_to_num($env).unwrap().num_val );
}

fn disp_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
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

fn new_oper_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   /* constants */
   let sigil_sym = rc_obj!(SYM; "sigil");
   let sigil_num = rc_obj!(NUM; 0);
   let rhs_sym   = rc_obj!(SYM; "rhs");
   let rhs_num   = rc_obj!(NUM; 1);
   let lhs_sym   = rc_obj!(SYM; "lhs");
   let lhs_num   = rc_obj!(NUM; 2);
   let prior_sym = rc_obj!(SYM; "priority");
   let prior_num = rc_obj!(NUM; 3);
   let func_sym  = rc_obj!(SYM; "func");
   let func_num  = rc_obj!(NUM; 4);

   /* attempt to find args */
   let sigil_arg = get_arg!(args, env, sigil_sym, 
                   get_arg!(args, env, sigil_num; Stack, panic!("Can't find sigil")));
   let rhs_arg   = get_arg!(args, env, rhs_sym, 
                   get_arg!(args, env, rhs_num; Stack, panic!("Can't find rhs")));
   let lhs_arg   = get_arg!(args, env, lhs_sym, 
                   get_arg!(args, env, lhs_num; Stack, panic!("Can't find lhs")));
   let prior_arg = get_arg!(args, env, prior_sym, 
                   get_arg!(args, env, prior_num; Stack, panic!("Can't find priority")));
   let func_arg  = get_arg!(args, env, func_sym, 
                   get_arg!(args, env, func_num; Stack, panic!("Can't find func")));
   
   /* convert to types required by Operator::new */
   let sigil    = to_type!(TEXT; sigil_arg, env);
   let lhs      = to_type!(BOOL; lhs_arg, env);
   let rhs      = to_type!(BOOL; rhs_arg, env);
   let priority = to_type!(NUM;  prior_arg, env) as u32;
   let func = OperFunc::Callable(func_arg);

   /* Create oper and assign it */
   let oper = Operator::new(rc!(sigil.clone()), lhs, rhs, priority, func);
   // Ok(rc!(oper))
   env.universe.set(sigil_arg, rc!(oper), AccessType::Locals)
}

fn if_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let cond_s_sym  = rc_obj!(NUM; 0);
   let cond_l_sym  = rc_obj!(SYM; "cond");
   let true_s_sym  = rc_obj!(NUM; 1);
   let true_l_sym  = rc_obj!(SYM; "if_true");
   let false_s_sym = rc_obj!(NUM; 2);
   let false_l_sym = rc_obj!(SYM; "if_false");

   let cond_arg = get_arg!(args, env, cond_s_sym; Stack, 
                  get_arg!(args, env, cond_l_sym; Locals, panic!("No condition!")));
   let true_arg = get_arg!(args, env, true_s_sym; Stack, 
                  get_arg!(args, env, true_l_sym; Locals, panic!("No true block!")));
   let false_arg = get_arg!(args, env, false_s_sym; Stack, 
                   get_arg!(args, env, false_l_sym; Locals, rc!(boolean::NULL)));
   let cond = to_type!(BOOL; cond_arg, env);
   if cond {
      Ok(true_arg)
   } else {
      Ok(false_arg)
   }
}

fn while_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let cond_s_sym  = rc_obj!(NUM; 0);
   let cond_l_sym  = rc_obj!(SYM; "cond");
   let body_s_sym  = rc_obj!(NUM; 1);
   let body_l_sym  = rc_obj!(SYM; "body");

   let cond_arg = get_arg!(args, env, cond_s_sym; Stack, 
                  get_arg!(args, env, cond_l_sym; Locals, panic!("No condition!")));
   let body_arg = get_arg!(args, env, body_s_sym; Stack, 
                  get_arg!(args, env, body_l_sym; Locals, panic!("No body block!")));

   let cond = cond_arg;
   let body = body_arg;

   loop {
      match cond.clone().qt_exec(env){
         Ok(obj) => match obj.qt_get(rc_obj!(NUM; 0), AccessType::Stack, env) {
            Ok(obj) => if to_type!(BOOL; obj, env) {
                          cast_as!(body, Universe).clone().exec(env);
                       } else {
                          break
                       },
            Err(err) => panic!("While condition returned error: {:?}", err)
         },
         Err(err) => panic!("Howto error?: {:?}", err) 
      }
   }
   ok_rc!(boolean::NULL)
}


fn literal_fn(args: Rc<&Universe>, env: &mut Environment) -> ObjResult {
   let mut oper_acc = String::new();
   for arg in args.stack.iter() {
      oper_acc += arg.qt_to_text(env).unwrap().text_val.as_str();
   }
   let oper_acc_dup = oper_acc.clone();
   let oper_sym = rc!(Symbol::new(oper_acc));
   match env.universe.get(oper_sym, AccessType::NonStack) {
      Ok(obj) => Ok(obj),
      Err(ObjError::NoSuchKey) => panic!("No such literal found: {:?}", oper_acc_dup),
      Err(err) => panic!("TODO: Handle this error: {:?}", err)
   }
}

pub fn functions() -> BuiltinsType {
   macro_rules! rc_func {
       ($func:ident) => (rc!(BuiltinFunction::new($func)))
   }
   map! { TYPE; BuiltinsType,
      "disp" => rc_func!(disp_fn),
      "new_oper" => rc_func!(new_oper_fn),
      "literal" => rc_func!(literal_fn),
      "if" => rc_func!(if_fn),
      "while" => rc_func!(while_fn)
   }
}










