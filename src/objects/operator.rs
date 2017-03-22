use env::Environment;
use objects::obj_rc::ObjRc;
use objects::universe::AccessType;

use parser::TokenPair;
use objects::text::Text;
use std::rc::Rc;
use objects::object::{Object, ObjType};
use objects::universe::Universe;
use objects::number::Number;
use objects::boolean::Boolean;
use objects::single_character::SingleCharacter;

use plugins::plugin::Plugin;
use plugins::operator_plugin;

use result::{ObjResult, ObjError, BoolResult};


macro_rules! oper_func {
    (BINARY: $name:ident, $name_l:ident, $name_r:ident, $res_type:ty ) => {

         fn $name(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> $res_type {
            let l = l.unwrap();
            let r = r.unwrap();
            match l.$name_l(&r, env) {
               Ok(e) => Ok(e),
               Err(ObjError::NotImplemented) => r.$name_r(&l, env),
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
            let lhs_sym = rc!(Symbol::from("lhs"));
            let rhs_sym = rc!(Symbol::from("rhs"));
            let mut args = env.universe.to_globals();
            if let Some(l) = l { args.set(lhs_sym, l, AccessType::Locals); }
            if let Some(r) = r { args.set(rhs_sym, r, AccessType::Locals); }
            match uni.qt_call(rc!(args), env) {
               Ok(obj) => obj.qt_get(rc!(Number::new((cast_as!(obj, Universe).stack.len() - 1) as i32)), AccessType::Stack, env),
               Err(err) => Err(err)
            }
            
         }
      }
   }
}
pub struct Operator {
   pub sigil: Rc<String>,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub priority: u32,
   pub func: OperFunc,
}

use std::cmp::{Ord, PartialOrd, Ordering};
impl PartialEq for Operator {
   fn eq(&self, other: &Operator) -> bool {
      self.sigil == other.sigil
   }
}
impl Eq for Operator {}
impl PartialOrd for Operator {
  fn partial_cmp(&self, other: &Operator) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}
impl Ord for Operator {
   fn cmp(&self, other: &Operator) -> Ordering {
      self.sigil.cmp(&other.sigil)
   }
}
impl Operator {
   pub fn new(sigil: Rc<String>,
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


oper_func!(BINARY: qt_add, qt_add_l, qt_add_r, ObjResult);
oper_func!(BINARY: qt_sub, qt_sub_l, qt_sub_r, ObjResult);
oper_func!(BINARY: qt_mul, qt_mul_l, qt_mul_r, ObjResult);
oper_func!(BINARY: qt_div, qt_div_l, qt_div_r, ObjResult);
oper_func!(BINARY: qt_mod, qt_mod_l, qt_mod_r, ObjResult);
oper_func!(BINARY: qt_pow, qt_pow_l, qt_pow_r, ObjResult);

oper_func!(BINARY: qt_eql, qt_eql_l, qt_eql_r, BoolResult);
oper_func!(BINARY: qt_neq, qt_neq_l, qt_neq_r, BoolResult);
oper_func!(BINARY: qt_gth, qt_gth_l, qt_gth_r, BoolResult);
oper_func!(BINARY: qt_lth, qt_lth_l, qt_lth_r, BoolResult);
oper_func!(BINARY: qt_geq, qt_geq_l, qt_geq_r, BoolResult);
oper_func!(BINARY: qt_leq, qt_leq_l, qt_leq_r, BoolResult);

oper_func!(BINARY: qt_cmp, qt_cmp_l, qt_cmp_r, ObjResult);
oper_func!(BINARY: qt_rgx, qt_rgx_l, qt_rgx_r, ObjResult);
// make one unary for der

fn exec_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.unwrap().qt_exec(env)
}

fn endl_fn(l: Option<ObjRc>, r: Option<ObjRc>, _: &mut Environment) -> ObjResult {
   Err(ObjError::NoResultDontFail)
}
fn sep_fn(l: Option<ObjRc>, r: Option<ObjRc>, _: &mut Environment) -> ObjResult {
   Ok(l.unwrap())
}
fn assign_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   env.universe.set(l.unwrap(), r.unwrap(), AccessType::Locals)
}
fn deref_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   env.universe.get(l.unwrap(), AccessType::NonStack)
}
fn get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.unwrap().qt_get(r.unwrap(), AccessType::All, env)
}
fn call_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.unwrap().qt_call(r.unwrap(), env)
}
fn and_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();
   let l_bool = match l.qt_to_bool(env) {
      Ok(obj) => obj.bool_val,
      Err(ObjError::NotImplemented) => true,
      Err(err) => panic!("unimplemented for error: {:?}", err)
   };
   match l_bool {
      true => Ok(r.unwrap()),
      false => Ok(l),
   }
}

fn or_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();
   let l_bool = match l.qt_to_bool(env) {
      Ok(obj) => obj.bool_val,
      Err(ObjError::NotImplemented) => true,
      Err(err) => panic!("unimplemented for error: {:?}", err)
   };
   match l_bool {
      true => Ok(l),
      false => Ok(r.unwrap())
   }
}

fn debug_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   // let mut forked = &mut env.fork(None, None, None);
   // let TokenPair(token, _) = env.parser.next_object(forked);
   panic!()
   // token
}

use objects::universe::GlobalsType;
use objects::obj_rc::ObjRcWrapper;
use objects::symbol::Symbol;

pub fn operators() -> GlobalsType {
   macro_rules! new_oper {
      ($sigil:expr, $priority:expr, $func:ident) => {
         rc!(Operator::new( rc!($sigil.to_string()), true, true, $priority, OperFunc::Function(rc!($func))))
      };
      ($sigil:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr) => {
         rc!(Operator::new( rc!($sigil.to_string()), $has_lhs, $has_rhs, $priority, OperFunc::Function(rc!($func))))
      }
   }
   
   map! { TYPE; GlobalsType,
      "+" => new_oper!("+", 12, qt_add),
      "-" => new_oper!("-", 12, qt_sub),
      "*" => new_oper!("*", 11, qt_mul),
      "/" => new_oper!("/", 11, qt_div),
      "%" => new_oper!("%", 11, qt_mod),
      // "**" => // new_oper!("**", 10, qt_pow),

      "&&" => new_oper!("&&",  24, and_fn),
      "||" => new_oper!("||",  25, or_fn),

      "," => new_oper!(",", 40, sep_fn, true, false),
      ";" => new_oper!(";", 40, endl_fn, true, false),
      "@" => new_oper!("@",  7, call_fn),
      "=" => new_oper!("=", 35, assign_fn),
      "?" => new_oper!("?",  1, deref_fn, true, false),
      "!" => new_oper!("!",  1, exec_fn, true, false),
      // "$" => new_oper!("$",  2, debug_fn, false, false),
      "." => new_oper!(".",  5, get_fn),
      "<-" => new_oper!("<-", 30, assign_fn),
      ":" => new_oper!("<-", 36, assign_fn)
   }
}


impl Operator {
   pub fn call_oper(&self, l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) {
      match self.func.call_oper(l, r, env) {
         Ok(obj) => env.universe.push(obj),
         Err(ObjError::NoResultDontFail) => {},
         Err(ObjError::NotImplemented) => panic!("Operator {:?} not implemented", self),
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
   obj_functions!(QT_EQL; Operator, sigil);
   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      operator_plugin::INSTANCE.handle(rc!(self.clone()), env);
      Err(ObjError::NoResultDontFail)
   }
}


impl Clone for Operator{
   fn clone(&self) -> Operator {
      Operator{sigil: self.sigil.clone(),
               priority: self.priority.clone(),
               has_lhs: self.has_lhs.clone(),
               has_rhs: self.has_rhs.clone(),
               func: self.func.clone()}
   }
}
impl_defaults!(DISPLAY_DEBUG; Operator, 'O');




