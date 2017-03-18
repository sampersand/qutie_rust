use objects::object::{Object, ObjType};
use objects::boxed_obj::BoxedObj;
use objects::boolean::Boolean;
use objects::universe::{Universe, AccessType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;
use environment::Environment;


use result::{ObjResult, ObjError};


macro_rules! oper_func {
    ( $name:ident, $name_l:ident, $name_r:ident ) => {

         fn $name(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> ObjResult {
            let l = l.unwrap();
            let r = r.unwrap();
            match l.$name_l(&r) {
               Ok(e) => Ok(e),
               Err(ObjError::NotImplemented) => panic!("TODO: rhs"),
               Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
            }
         }
    };
}

macro_rules! new_oper {
   ($sigil:expr, $priority:expr, $func:ident) => {
      Operator{
         sigil: $sigil,
         priority: $priority,
         has_lhs: true,
         has_rhs: true,
         func: $func
      };
   };
   ($sigil:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr) => {
      Operator{
         sigil: $sigil,
         priority: $priority,
         has_lhs: $has_lhs,
         has_rhs: $has_rhs,
         func: $func
      };
   }
}

pub struct Operator {
   pub sigil: &'static str,
   pub priority: u32,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub func: fn(Option<BoxedObj>, Option<BoxedObj>, &mut Environment) -> ObjResult,
}



oper_func!(qt_add, qt_add_l, qt_add_r);
oper_func!(qt_sub, qt_sub_l, qt_sub_r);
oper_func!(qt_mul, qt_mul_l, qt_mul_r);
oper_func!(qt_div, qt_div_l, qt_div_r);
oper_func!(qt_mod, qt_mod_l, qt_mod_r);
oper_func!(qt_pow, qt_pow_l, qt_pow_r);

oper_func!(qt_eql, qt_eql_l, qt_eql_r);
oper_func!(qt_neq, qt_neq_l, qt_neq_r);
oper_func!(qt_gth, qt_gth_l, qt_gth_r);
oper_func!(qt_lth, qt_lth_l, qt_lth_r);
oper_func!(qt_geq, qt_geq_l, qt_geq_r);
oper_func!(qt_leq, qt_leq_l, qt_leq_r);

oper_func!(qt_cmp, qt_cmp_l, qt_cmp_r);
oper_func!(qt_rgx, qt_rgx_l, qt_rgx_r);


fn endl_fn(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> ObjResult {
   assert_eq!(r, None);
   Err(ObjError::NoResultDontFail)
}
fn sep_fn(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> ObjResult {
   assert_eq!(r, None);
   let l = l.unwrap();
   Ok(l)
}
fn assign_fn(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> ObjResult {
   let l = l.unwrap();
   let r = r.unwrap();
   env.universe.set(l, r, AccessType::Locals);
   Ok(Box::new(Boolean::Null))
}
fn deref_fn(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> ObjResult {
   assert_eq!(r, None);
   let l = l.unwrap();
   // env.universe.get(l, AccessType::Locals)
   panic!("TODO: THIS")
}

lazy_static! {
    pub static ref OPERATORS: Vec<Operator> = vec![
      new_oper!("+", 12, qt_add),
      new_oper!("-", 12, qt_sub),
      new_oper!("*", 11, qt_mul),
      new_oper!("/", 11, qt_div),
      new_oper!("%", 11, qt_mod),
      // new_oper!("**", 10, qt_pow),
      new_oper!(",", 40, sep_fn, true, false),
      new_oper!(";", 40, endl_fn, true, false),
      new_oper!("=", 35, assign_fn),
      new_oper!("?",  1, deref_fn, true, false),
    ];
}


impl Clone for Operator{
   fn clone(&self) -> Operator {
      Operator{sigil: self.sigil.clone(),
               priority: self.priority.clone(),
               has_lhs: self.has_lhs.clone(),
               has_rhs: self.has_rhs.clone(),
               func: self.func}
   }
}

impl Object for Operator {
   fn obj_type(&self) -> ObjType { ObjType::Operator(self) }
   fn source(&self) -> Vec<SingleCharacter>{
      let mut ret = vec![];
      for chr in self.sigil.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
}

impl Display for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.sigil)
   }
}
impl Debug for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "O({})", self)
   }
}