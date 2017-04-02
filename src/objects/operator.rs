use env::Environment;
use objects::obj_rc::ObjRc;
use objects::universe::AccessType;

use parser::TokenPair;
use objects::text::Text;
use std::rc::Rc;
use objects::object::{Object, ObjType, ObjWrapper};
use objects::universe::Universe;
use objects::number::Number;
use objects::boolean::Boolean;
use objects::single_character::SingleCharacter;

use plugins::plugin::Plugin;
use plugins::operator_plugin;
use objects::user_function::UserFunction;
use result::{ObjResult, ObjError};


macro_rules! oper_func {
    (BINARY: $name:ident, $name_l:ident, $name_r:ident ) => {

         fn $name(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
            let l = l.unwrap();
            let r = r.unwrap();
            match l.$name_l(r.clone(), env) {
               Ok(e) => Ok(e),
               Err(ObjError::NotImplemented) => r.$name_r(l, env),
               Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
            }
         }
    };
}

#[derive(Clone)]
pub enum OperFunc {
   Function(Rc<fn(Option<ObjRc>, Option<ObjRc>, &mut Environment) -> ObjResult>),
   Callable(Rc<Object>)
}

impl OperFunc {
   fn call_oper(&self, l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
      match *self {
         OperFunc::Function(ref func) => (func)(l, r, env),
         OperFunc::Callable(ref uni) => {
            let mut args = env.universe.to_globals();
            if l.is_some() {
               let lhs_sym = new_obj!(SYM_STATIC, "lhs");
               args.set(lhs_sym, l.unwrap(), AccessType::Locals);
            }
            if r.is_some() {
               let rhs_sym = new_obj!(SYM_STATIC, "rhs");
               args.set(rhs_sym, r.unwrap(), AccessType::Locals);
            }
            uni.qt_call(rc!(args), env)
         }
      }
   }
}

pub struct Operator {
   pub sigil: String,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub priority: u32,
   pub func: OperFunc,
}
impl PartialEq for Operator {
   fn eq(&self, other: &Operator) -> bool {
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
         sigil: sigil,
         has_lhs: has_lhs,
         has_rhs: has_rhs,
         priority: priority,
         func: func
      }
   }
}


oper_func!(BINARY: qt_add, qt_add_l, qt_add_r);
oper_func!(BINARY: qt_sub, qt_sub_l, qt_sub_r);
oper_func!(BINARY: qt_mul, qt_mul_l, qt_mul_r);
oper_func!(BINARY: qt_div, qt_div_l, qt_div_r);
oper_func!(BINARY: qt_mod, qt_mod_l, qt_mod_r);
oper_func!(BINARY: qt_pow, qt_pow_l, qt_pow_r);

oper_func!(BINARY: qt_eql, qt_eql_l, qt_eql_r);
oper_func!(BINARY: qt_neq, qt_neq_l, qt_neq_r);
oper_func!(BINARY: qt_gth, qt_gth_l, qt_gth_r);
oper_func!(BINARY: qt_lth, qt_lth_l, qt_lth_r);
oper_func!(BINARY: qt_geq, qt_geq_l, qt_geq_r);
oper_func!(BINARY: qt_leq, qt_leq_l, qt_leq_r);

oper_func!(BINARY: qt_cmp, qt_cmp_l, qt_cmp_r);
oper_func!(BINARY: qt_rgx, qt_rgx_l, qt_rgx_r);
// make one unary for der

pub fn exec_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert!(r.is_none());
   l.unwrap().qt_exec(env)
}

fn endl_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert!(l.is_none());
   assert!(r.is_none());
   env.universe.stack.pop();
   Err(ObjError::NoResultDontFail)
}
fn sep_fn(l: Option<ObjRc>, r: Option<ObjRc>, _: &mut Environment) -> ObjResult {
   assert!(l.is_none());
   assert!(r.is_none());
   Err(ObjError::NoResultDontFail)
}

fn assign_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let r = r.unwrap();
   env.universe.set(l.unwrap(), r.clone(), AccessType::Locals);
   Ok(r)
}

pub fn deref_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   assert!(r.is_none());
   env.universe.get(l.unwrap(), AccessType::NonStack)
}

fn get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();
   let res = l.clone().qt_get(r.unwrap(), env).unwrap();
   if res.is_a(ObjType::UserFunction) {
      cast_as!(CL; res, UserFunction).set_parent(cast_as!(l, Universe))
   }
   Ok(res)
   
}

fn set_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let lhs = l.unwrap();
   let rhs = cast_as!(r.unwrap(), Universe);
   let key = rhs.get(new_obj!(NUM, 1), AccessType::Stack).unwrap();
   let val = rhs.get(new_obj!(NUM, 0), AccessType::Stack).unwrap();
   let mut lhs: &mut Object = unsafe {
      use std::mem::transmute;
      #[allow(mutable_transmutes)]
      transmute(&*lhs)
   };
   lhs.qt_set(key, val, env)

}

pub fn call_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.unwrap().qt_call(r.unwrap(), env)
}

fn call_get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   cast_as!(call_fn(l, r, env).unwrap(), Universe).get(new_obj!(NUM, 0), AccessType::Stack)
}

fn and_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();
   if to_type!(BOOL; l, env) {
      Ok(r.unwrap())
   }  else {
      Ok(l)
   }
}

fn or_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();

   if to_type!(BOOL; l, env) {
      Ok(l)
   }  else {
      Ok(r.unwrap())
   }
}


use objects::universe::GlobalsType;
use objects::obj_rc::ObjRcWrapper;
use objects::symbol::Symbol;

pub fn operators() -> GlobalsType {

   macro_rules! new_oper {
      ($sigil:expr, $priority:expr, $func:ident) => {
         rc!(Operator::new( $sigil.to_string(), true, true, $priority, OperFunc::Function(rc!($func))))
      };
      ($sigil:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr) => {
         rc!(Operator::new( $sigil.to_string(), $has_lhs, $has_rhs, $priority, OperFunc::Function(rc!($func))))
      }
   }
   map! { TYPE; GlobalsType,
      ","  => new_oper!(",",  100, sep_fn, false, false),
      ";"  => new_oper!(";",  100, endl_fn, false, false),
      "="  => new_oper!("=",  90,  assign_fn),
      ".=" => new_oper!(".=", 90,  set_fn),

      /* gap here is for user-defined opers */ 
      "||"  => new_oper!("||",  48, or_fn),
      "&&"  => new_oper!("&&",  47, and_fn),

      "<>" => new_oper!("<>", 46, qt_neq),
      "==" => new_oper!("==", 46, qt_eql),
      "<=>"=> new_oper!("<=>",45, qt_cmp),
      "<"  => new_oper!("<",  144, qt_lth),
      ">"  => new_oper!(">",  44, qt_gth),
      "<=" => new_oper!("<=", 44, qt_leq),
      ">=" => new_oper!(">=", 44, qt_geq),

      "+" => new_oper!("+", 35, qt_add),
      "-" => new_oper!("-", 35, qt_sub),
      "*" => new_oper!("*", 34, qt_mul),
      "/" => new_oper!("/", 34, qt_div),
      "%" => new_oper!("%", 34, qt_mod),
      // "**" => // new_oper!("**", 33, qt_pow),

      "@" => new_oper!("@", 20, call_fn),
      "@0" => new_oper!("@0", 20, call_get_fn),
      "." => new_oper!(".", 5, get_fn),
      "?" => new_oper!("?", 1, deref_fn, true, false),
      "!" => new_oper!("!", 1, exec_fn, true, false)
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
      operator_plugin::INSTANCE.handle(rc!(self.clone()), env);
      Err(ObjError::NoResultDontFail)
   }
}

impl_defaults!(DISPLAY_DEBUG; Operator, 'O');




