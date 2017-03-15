use std::fmt::{Debug, Display};
use objects::single_character;
use objects::operator;
use objects::boxed_obj::BoxedObj;
use objects::number;
use objects::text;
use objects::symbol;
use result::{ObjResult, ObjError, BoolResult};

#[derive(Debug)]
pub enum ObjType<'a> {
   Universe,
   Number(&'a number::Number),
   SingleCharacter(&'a single_character::SingleCharacter),
   Symbol(&'a symbol::Symbol),
   Text,
   Boolean,
   Operator(&'a operator::Operator),
}

macro_rules! default_func {
   (SINGLE: $name:ident) => {
      fn $name(&self) -> ObjResult { Err(ObjError::NotImplemented) }
   };
   (OBJ: $name:ident, $name_l:ident, $name_r:ident) => {
      fn $name(&self, other: &BoxedObj) -> ObjResult {
         match self.$name_l(other) {
            Err(ObjError::NotImplemented) => self.$name_r(other),
            e @ _ => e
         }
      }
      fn $name_l(&self, other: &BoxedObj) -> ObjResult { Err(ObjError::NotImplemented) }
      fn $name_r(&self, other: &BoxedObj) -> ObjResult { Err(ObjError::NotImplemented) }
   };
   (BOOL: $name:ident, $name_l:ident, $name_r:ident) => {
      fn $name(&self, other: &BoxedObj) -> BoolResult {
         match self.$name_l(other) {
            Err(ObjError::NotImplemented) => self.$name_r(other),
            e @ _ => e
         }
      }
      fn $name_l(&self, other: &BoxedObj) -> BoolResult { Err(ObjError::NotImplemented) }
      fn $name_r(&self, other: &BoxedObj) -> BoolResult { Err(ObjError::NotImplemented) }
   };
}



pub trait Object : Debug + Display {
   fn obj_type(&self) -> ObjType;
   fn source(&self) -> Vec<single_character::SingleCharacter>;

   default_func!(SINGLE: qt_to_num);
   default_func!(SINGLE: qt_to_to_text);


   default_func!(OBJ: qt_add, qt_add_l, qt_add_r);
   default_func!(OBJ: qt_sub, qt_sub_l, qt_sub_r);
   default_func!(OBJ: qt_mul, qt_mul_l, qt_mul_r);
   default_func!(OBJ: qt_div, qt_div_l, qt_div_r);
   default_func!(OBJ: qt_mod, qt_mod_l, qt_mod_r);
   default_func!(OBJ: qt_pow, qt_pow_l, qt_pow_r);

   default_func!(BOOL: qt_eql, qt_eql_l, qt_eql_r);
   default_func!(BOOL: qt_neq, qt_neq_l, qt_neq_r);
   default_func!(BOOL: qt_gth, qt_gth_l, qt_gth_r);
   default_func!(BOOL: qt_lth, qt_lth_l, qt_lth_r);
   default_func!(BOOL: qt_leq, qt_leq_l, qt_leq_r);
   default_func!(BOOL: qt_geq, qt_geq_l, qt_geq_r);
   
   default_func!(OBJ: qt_cmp, qt_cmp_l, qt_cmp_r);
   default_func!(OBJ: qt_rgx, qt_rgx_l, qt_rgx_r);

}







