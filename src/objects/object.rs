use std::fmt::{Debug, Display};
use std::rc::Rc;
use objects::{single_character, operator, number, text, universe, symbol, boolean};
use objects::universe::AccessType;
use objects::obj_rc::ObjRc;
use result::{ObjResult, ObjError, BoolResult};
use env::Environment;

#[derive(Debug)]
pub enum ObjType<'a> {
   Universe(&'a universe::Universe),
   Number(&'a number::Number),
   SingleCharacter(&'a single_character::SingleCharacter),
   Symbol(&'a symbol::Symbol),
   Text,
   Boolean,
   Operator(&'a operator::Operator),
}

macro_rules! default_func {
   (UNARY: $name:ident, $ret_type:ty) => {
      fn $name(&self, env: &mut Environment) -> $ret_type { Err(ObjError::NotImplemented) }
   };
   (BINARY: $name:ident, $name_l:ident, $name_r:ident, $ret_type:ty) => {
      fn $name(&self, other: &ObjRc, env: &mut Environment) -> $ret_type {
         match self.$name_l(other, env) {
            Err(ObjError::NotImplemented) => self.$name_r(other, env),
            e @ _ => e
         }
      }
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> $ret_type { Err(ObjError::NotImplemented) }
      fn $name_r(&self, other: &ObjRc, env: &mut Environment) -> $ret_type { Err(ObjError::NotImplemented) }
   };
}


pub trait Object : Debug + Display {
   fn obj_type(&self) -> ObjType;
   fn source(&self) -> Vec<single_character::SingleCharacter>;

   default_func!(UNARY: qt_to_bool, Result<Rc<boolean::Boolean>, ObjError>);
   default_func!(UNARY: qt_to_num, Result<Rc<number::Number>, ObjError>);
   default_func!(UNARY: qt_to_text, Result<Rc<text::Text>, ObjError>);

   fn qt_exec(&self, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }

   default_func!(BINARY: qt_add, qt_add_l, qt_add_r, ObjResult); // is &ObjRc really needed, can't it be ObjRc
   default_func!(BINARY: qt_sub, qt_sub_l, qt_sub_r, ObjResult);
   default_func!(BINARY: qt_mul, qt_mul_l, qt_mul_r, ObjResult);
   default_func!(BINARY: qt_div, qt_div_l, qt_div_r, ObjResult);
   default_func!(BINARY: qt_mod, qt_mod_l, qt_mod_r, ObjResult);
   default_func!(BINARY: qt_pow, qt_pow_l, qt_pow_r, ObjResult);

   default_func!(BINARY: qt_eql, qt_eql_l, qt_eql_r, BoolResult);
   default_func!(BINARY: qt_neq, qt_neq_l, qt_neq_r, BoolResult);
   default_func!(BINARY: qt_gth, qt_gth_l, qt_gth_r, BoolResult);
   default_func!(BINARY: qt_lth, qt_lth_l, qt_lth_r, BoolResult);
   default_func!(BINARY: qt_leq, qt_leq_l, qt_leq_r, BoolResult);
   default_func!(BINARY: qt_geq, qt_geq_l, qt_geq_r, BoolResult);
   
   default_func!(BINARY: qt_cmp, qt_cmp_l, qt_cmp_r, ObjResult);
   default_func!(BINARY: qt_rgx, qt_rgx_l, qt_rgx_r, ObjResult);

   fn qt_get(&self, other: ObjRc, access_type: AccessType, env: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_call(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   // fn qt_get_l(&self, other: ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }
   // fn qt_get_r(&self, other: ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }


}









