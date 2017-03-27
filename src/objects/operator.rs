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

use result::{ObjResult, ObjError};


macro_rules! oper_func {
    (BINARY: $name:ident, $name_l:ident, $name_r:ident ) => {

         fn $name(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
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


pub enum OperFunc {
   Function(fn(Option<ObjRc>, Option<ObjRc>, &mut Environment) -> ObjResult),
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
   pub sigil: String,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub priority: u32,
   pub func: OperFunc,
}
// impl Clone for Operator {
//    fn clone(&self) -> Operator {
//       Operator::new(self.sigil.clone(),
//                     self.has_lhs,
//                     self.has_rhs,
//                     self.priority,
//                     self.func)
//    }
// }

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
   l.unwrap().qt_exec(env)
}

fn endl_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   env.universe.stack.pop();
   Err(ObjError::NoResultDontFail)
}
fn sep_fn(l: Option<ObjRc>, r: Option<ObjRc>, _: &mut Environment) -> ObjResult {
   Err(ObjError::NoResultDontFail)
}
fn assign_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   env.universe.set(l.unwrap(), r.unwrap(), AccessType::Locals)
}
pub fn deref_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   env.universe.get(l.unwrap(), AccessType::NonStack)
}
fn get_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   l.unwrap().qt_get(r.unwrap(), AccessType::All, env)
}
fn set_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
   // l.unwrap().qt_get(r.unwrap(), AccessType::All, env)
   panic!("{:?}.={:?}", l, r);
}
pub fn call_fn(l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) -> ObjResult {
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

pub static mut SET_OPER: *mut Operator = 0 as *mut Operator;
pub static mut SET_OPER_2: *const Operator = 0 as *const Operator;
pub static mut WAS_SET_OPER_SET: bool = false;
pub fn operators() -> GlobalsType {
   macro_rules! new_oper {
      ($sigil:expr, $priority:expr, $func:ident) => {
         rc!(Operator::new( $sigil.to_string(), true, true, $priority, OperFunc::Function($func)))
      };
      ($sigil:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr) => {
         rc!(Operator::new( $sigil.to_string(), $has_lhs, $has_rhs, $priority, OperFunc::Function($func)))
      }
   }
   unsafe {
      if !WAS_SET_OPER_SET {
         WAS_SET_OPER_SET = true;
      // if SET_OPER as *const Operator == 0 as *const Operator {
         println!("SET_OPER == 0: {:?}", SET_OPER);
         SET_OPER_2 = &Operator::new(".=".to_string(), true, true, 90, OperFunc::Function(set_fn)) as *const Operator;

         // *SET_OPER_2 = &mut oper as *mut *const Operator;

         println!("SET_OPER_2 <> 0: {:?}", SET_OPER_2);
         println!("SET_OPER_2 <> 0: {:?}", (*SET_OPER_2).sigil);
         println!("SET_OPER_2 <> 0: {:?}", (*SET_OPER_2).has_lhs);
         println!("SET_OPER_2 <> 0: {:?}", (*SET_OPER_2).has_rhs);
         println!("SET_OPER_2 <> 0: {:?}", (*SET_OPER_2).priority);
      } else {
         println!("SET_OPER_2 ?= 0: {:?}", SET_OPER_2);
         println!("SET_OPER_2 ?= 0: {:?}", WAS_SET_OPER_SET);
         let ref oper: Operator = *SET_OPER_2;
         println!("SET_OPER_2 != 0: {:?}", oper.sigil);
         println!("SET_OPER_2 != 0: {:?}", oper.has_lhs);
         println!("SET_OPER_2 != 0: {:?}", oper.has_rhs);
         println!("SET_OPER_2 != 0: {:?}", oper.priority);
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
      "<=>" => new_oper!("<=>",  45, qt_cmp),
      "<"  => new_oper!("<",  44, qt_lth),
      ">"  => new_oper!(">",  44, qt_gth),
      "<=" => new_oper!("<=", 44, qt_leq),
      ">=" => new_oper!(">=", 44, qt_geq),

      "+" => new_oper!("+", 35, qt_add),
      "-" => new_oper!("-", 35, qt_sub),
      "*" => new_oper!("*", 34, qt_mul),
      "/" => new_oper!("/", 34, qt_div),
      "%" => new_oper!("%", 34, qt_mod),
      // "**" => // new_oper!("**", 33, qt_pow),


      "@" => new_oper!("@",  20, call_fn),
      "." => new_oper!(".",  5, get_fn),
      "?" => new_oper!("?",  1, deref_fn, true, false),
      "!" => new_oper!("!",  1, exec_fn, true, false)
   }
}


impl Operator {
   pub fn call_oper(&self, l: Option<ObjRc>, r: Option<ObjRc>, env: &mut Environment) {
      let l_clone = l.clone();
      let r_clone = r.clone();
      match self.func.call_oper(l, r, env) {
         Ok(obj) => env.universe.push(obj),
         Err(ObjError::NoResultDontFail) => {},
         Err(ObjError::NotImplemented) => panic!("Operator {:?} not implemented for {:?} and {:?}",
                                                 self, l_clone, r_clone),
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
      // operator_plugin::INSTANCE.handle(rc!(self.clone()), env);
      panic!("TODO: EXEC OPERATOR");
      Err(ObjError::NoResultDontFail)
   }
}

impl_defaults!(DISPLAY_DEBUG; Operator, 'O');




