use globals::IdType;
use env::Environment;
use objects::obj_rc::ObjRc;
use objects::universe::AccessType;

use parser::TokenPair;
use objects::text::Text;
use std::rc::Rc;
use objects::object::{Object, ObjType, ObjWrapper};
use objects::universe::Universe;
use objects::number::Number;
use objects::boolean::{Boolean, BoolType};
use objects::single_character::SingleCharacter;

use plugins::plugin::Plugin;
use plugins::operator_plugin;
use objects::user_function::UserFunction;
use result::{ObjResult, ObjError, BoolResult};


macro_rules! oper_func {
    (BINARY: $name:ident, $ret_type:ty ) => {
         fn $name(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> $ret_type {
            l.expect("can't get lhs").$name(r.expect("can't get rhs"), env)
         }
    };
}

#[derive(Clone)]
pub enum OperFunc {
   FunctionObj(Rc<fn(Option<ObjRc>, Option<ObjRc>, &mut Environment) -> ObjResult>),
   FunctionBool(Rc<fn(Option<ObjRc>, Option<ObjRc>, &mut Environment) -> BoolResult>),
   Callable(Rc<Object>)
}

impl OperFunc {
   fn call_oper(&self, l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
      match *self {
         OperFunc::FunctionObj(ref func) => (func)(l, r, env),
         OperFunc::FunctionBool(ref func) => 
            match (func)(l, r, env) {
               Ok(obj) => Ok(obj),
               Err(obj) => Err(obj)
            },
         OperFunc::Callable(ref uni) => {
            let mut args = env.universe.to_globals();
            let lhs_sym = new_obj!(SYM_STATIC, "lhs");
            let rhs_sym = new_obj!(SYM_STATIC, "rhs");
            args.set(lhs_sym,
                     if l.is_some() {
                        l.expect("can't find lhs_sym")
                     } else {
                        new_obj!(BOOL_STATIC, Null)
                     },
                     AccessType::Locals);
            args.set(rhs_sym,
                     if r.is_some() {
                        r.expect("can't find rhs_sym")
                     } else {
                        new_obj!(BOOL_STATIC, Null)
                     },
                     AccessType::Locals);
            uni.qt_call(args.to_rc(), env)
         }
      }
   }
}

pub struct Operator {
   id: IdType,
   pub sigil: String,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub priority: u32,
   pub func: OperFunc,
}
impl PartialEq for Operator {
   fn eq(&self, other: &Operator) -> bool {
      if self.id == other.id{
         return true;
      }
      self.sigil == other.sigil && 
      self.has_lhs == other.has_lhs &&
      self.has_rhs == other.has_rhs && 
      self.priority == other.priority /*&& 
      self.func == other.func*/
   } 
}

impl Clone for Operator {
   fn clone(&self) -> Operator {
      Operator::new(self.sigil.clone(),
                    self.has_lhs,
                    self.has_rhs,
                    self.priority,
                    self.func.clone())
   }
}

impl Operator {
   pub fn new(sigil: String,
              has_lhs: bool,
              has_rhs: bool,
              priority: u32,
              func: OperFunc) -> Operator {
      Operator{
         id: next_id!(),
         sigil: sigil,
         has_lhs: has_lhs,
         has_rhs: has_rhs,
         priority: priority,
         func: func
      }
   }
   pub fn to_rc(self) -> Rc<Operator> {
      Rc::new(self)
   }
}


oper_func!(BINARY: qt_add, ObjResult);
oper_func!(BINARY: qt_sub, ObjResult);
oper_func!(BINARY: qt_mul, ObjResult);
oper_func!(BINARY: qt_div, ObjResult);
oper_func!(BINARY: qt_mod, ObjResult);
oper_func!(BINARY: qt_pow, ObjResult);

oper_func!(BINARY: qt_eql, BoolResult);
oper_func!(BINARY: qt_neq, BoolResult);
oper_func!(BINARY: qt_gth, BoolResult);
oper_func!(BINARY: qt_lth, BoolResult);
oper_func!(BINARY: qt_geq, BoolResult);
oper_func!(BINARY: qt_leq, BoolResult);

oper_func!(BINARY: qt_cmp, BoolResult);
oper_func!(BINARY: qt_rgx, ObjResult);
// make one unary for der

pub fn exec_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert_debug!(is_none; r);
   l.expect("no lhs for exec_fn").qt_exec(env)
}

fn endl_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert_debug!(is_none; l);
   assert_debug!(is_none; r);
   env.universe.stack.pop();
   Err(ObjError::NoResultDontFail)
}
fn sep_fn(l: Option<ObjRc>, r: Option<ObjRc>, _: &mut Environment) -> ObjResult {
   assert_debug!(is_none; l);
   assert_debug!(is_none; r);
   Err(ObjError::NoResultDontFail)
}

fn assign_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let r = r.expect("no rhs for assign_fn");
   env.universe.set(l.expect("no lhs for assign_fn"), r.clone(), AccessType::Locals);
   Ok(r)
}

pub fn deref_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert_debug!(is_none; r);
   env.universe.get(l.expect("no lhs for deref_fn"), AccessType::NonStack)
}

pub fn get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.expect("no lhs for get_fn");
   match l.clone().qt_get(r.expect("no rhs for get_fn"), env){
      Ok(res) => {
         if res.is_a(ObjType::UserFunction) {
            cast_as!(CL; res, UserFunction).set_parent(cast_as!(l, Universe))
         }
         Ok(res)
      },
      // Err(ObjError::NoSuchKey(_)) if l.is_a(ObjType::Universe) => {
      //    Ok(new_obj!(BOOL_STATIC, Null))
      // }
      Err(err) => Err(err)
   }
}

fn set_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let lhs = l.expect("no l for set_fn");
   let rhs = cast_as!(r.expect("no r for set_fn"), Universe);
   let key = rhs.get(new_obj!(NUM, 1), AccessType::Stack).expect("no index 1 (key) for set_fn");
   let val = rhs.get(new_obj!(NUM, 0), AccessType::Stack).expect("no index 0 (val) for set_fn");
   let mut var: &mut Object = unsafe {
      use std::mem::transmute;
      #[allow(mutable_transmutes)]
      transmute(&*lhs)
   };
   var.qt_set(key, val, env)

}

pub fn call_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.expect("no lhs for call_fn").qt_call(r.expect("no rhs for call_fn"), env)
}

fn call_get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   cast_as!(call_fn(l, r, env).expect("err call_get_fn"), Universe).get(new_obj!(NUM, 0), AccessType::Stack)
}

fn and_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.expect("no lhs for and_fn");
   if to_type!(BOOL; l, env) {
      Ok(r.expect("no rhs for and_fn"))
   }  else {
      Ok(l)
   }
}

fn or_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.expect("no lhs for or_fn");
   if to_type!(BOOL; l, env) {
      Ok(l)
   }  else {
      Ok(r.expect("no rhs for or_fn"))
   }
}


use objects::universe::GlobalsType;
use objects::obj_rc::ObjRcWrapper;
use objects::symbol::Symbol;

pub fn operators() -> GlobalsType {

   macro_rules! new_oper {
      ($sigil:expr, $priority:expr, $func:ident, $_type:ident) => {
         Operator::new($sigil.to_string(), true, true, $priority,
                       OperFunc::$_type(Rc::new($func))).to_rc()
      };
      ($sigil:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr, $_type:ident) => {
         Operator::new($sigil.to_string(), $has_lhs, $has_rhs, $priority,
                       OperFunc::$_type(Rc::new($func))).to_rc()
      }
   }
   map! { TYPE; GlobalsType,
      ","  => new_oper!(",",  100, sep_fn, false, false, FunctionObj),
      ";"  => new_oper!(";",  100, endl_fn, false, false, FunctionObj),
      "="  => new_oper!("=",  90,  assign_fn, FunctionObj),
      ".=" => new_oper!(".=", 90,  set_fn, FunctionObj),

      /* gap here is for user-defined opers */ 
      "||"  => new_oper!("||",  48, or_fn, FunctionObj),
      "&&"  => new_oper!("&&",  47, and_fn, FunctionObj),

      "!=" => new_oper!("!=", 46, qt_neq, FunctionBool),
      "<>" => new_oper!("<>", 46, qt_neq, FunctionBool),
      "==" => new_oper!("==", 46, qt_eql, FunctionBool),
      "<=>"=> new_oper!("<=>",45, qt_cmp, FunctionBool),
      "<"  => new_oper!("<",  144, qt_lth, FunctionBool),
      ">"  => new_oper!(">",  44, qt_gth, FunctionBool),
      "<=" => new_oper!("<=", 44, qt_leq, FunctionBool),
      ">=" => new_oper!(">=", 44, qt_geq, FunctionBool),

      "+" => new_oper!("+", 35, qt_add, FunctionObj),
      "-" => new_oper!("-", 35, qt_sub, FunctionObj),
      "*" => new_oper!("*", 34, qt_mul, FunctionObj),
      "/" => new_oper!("/", 34, qt_div, FunctionObj),
      "%" => new_oper!("%", 34, qt_mod, FunctionObj),
      // "**" => // new_oper!("**", 33, qt_pow, FunctionObj),

      "@" => new_oper!("@", 20, call_fn, FunctionObj),
      "@0" => new_oper!("@0", 20, call_get_fn, FunctionObj),
      "." => new_oper!(".", 3, get_fn, FunctionObj),
      "?" => new_oper!("?", 1, deref_fn, true, false, FunctionObj),
      "!" => new_oper!("!", 5, exec_fn, true, false, FunctionObj)
   }
}


impl Operator {
   pub fn call_oper(&self, l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) {
      let l_clone = l.clone();
      let r_clone = r.clone();
      match self.func.call_oper(l, r, env) {
         Ok(obj) => env.universe.push(obj),
         Err(ObjError::NoResultDontFail) => {},
         Err(ObjError::NotImplemented) => panic!("Operator {:?} not implemented for {:?} and {:?}", self, l_clone, r_clone),
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }

   pub fn to_string(&self) -> String {
      self.sigil.to_string()
   }
}

impl Object for Operator {
   impl_defaults!(OBJECT; Operator);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; sigil);
   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      operator_plugin::INSTANCE.handle(self.clone().to_rc(), env);
      Err(ObjError::NoResultDontFail)
   }
}

impl_defaults!(DISPLAY_DEBUG; Operator, 'O');




