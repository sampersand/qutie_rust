use std::fmt::{Debug, Display};
use std::rc::Rc;
use objects::{single_character, operator, number,
              text, universe, symbol, boolean,
              builtin_function,
              user_function, user_class};
use objects::universe::AccessType;
use objects::obj_rc::ObjRc;
use result::{ObjResult, ObjError};
use env::Environment;

#[derive(Debug)]
pub enum ObjType<'a> {
   Universe(&'a universe::Universe),
   Number(&'a number::Number),
   SingleCharacter(&'a single_character::SingleCharacter),
   Symbol(&'a symbol::Symbol),
   Text(&'a text::Text),
   Boolean(&'a boolean::Boolean),
   Operator(&'a operator::Operator),
   BuiltinFunction(&'a builtin_function::BuiltinFunction),
   UserFunction(&'a user_function::UserFunction),
   UserClass(&'a user_class::UserClass),
}

macro_rules! default_func {
   (UNARY: $name:ident, $ret_type:ty) => {
      fn $name(&self, env: &mut Environment) -> $ret_type { Err(ObjError::NotImplemented) }
   };
   (BINARY: $name:ident, $name_l:ident, $name_r:ident) => {
      fn $name(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
         match self.$name_l(other, env) {
            Err(ObjError::NotImplemented) => self.$name_r(other, env),
            other @ _ => other
         }
      }
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }
      fn $name_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }
   };
}


pub trait Object : Debug + Display {
   fn obj_type(&self) -> ObjType;
   fn source(&self) -> Vec<single_character::SingleCharacter>;

   default_func!(UNARY: qt_to_bool, Result<Rc<boolean::Boolean>, ObjError>);
   default_func!(UNARY: qt_to_num, Result<Rc<number::Number>, ObjError>);
   default_func!(UNARY: qt_to_text, Result<Rc<text::Text>, ObjError>);

   fn qt_method(&self, other: &str) -> ObjResult { Err(ObjError::NotImplemented) }

   fn qt_exec(&self, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }

   default_func!(BINARY: qt_add, qt_add_l, qt_add_r); // is &ObjRc really needed, can't it be ObjRc
   default_func!(BINARY: qt_sub, qt_sub_l, qt_sub_r);
   default_func!(BINARY: qt_mul, qt_mul_l, qt_mul_r);
   default_func!(BINARY: qt_div, qt_div_l, qt_div_r);
   default_func!(BINARY: qt_mod, qt_mod_l, qt_mod_r);
   default_func!(BINARY: qt_pow, qt_pow_l, qt_pow_r);

   fn qt_eql(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      match self.qt_eql_l(other, env) {
         Err(ObjError::NotImplemented) => self.qt_eql_r(other, env),
         other @ _ => other
      }
   }
   fn qt_eql_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult { ok_rc!(boolean::FALSE) }
   fn qt_eql_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult { ok_rc!(boolean::FALSE) }
   fn qt_neq(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      ok_rc!(boolean::TRUE)
      // match self.qt_neq_l(other, env) {
      //    Err(ObjError::NotImplemented) => self.qt_neq_r(other, env),
      //    other @ _ => other
      // }
   }
   fn qt_neq_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      let eql_other = self.qt_eql(other, env).unwrap().qt_to_bool(env).unwrap().bool_val;
      ok_rc!(boolean::Boolean::from_bool(!eql_other))
   }
   fn qt_neq_r(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
      let eql_other = self.qt_eql(other, env).unwrap().qt_to_bool(env).unwrap().bool_val;
      ok_rc!(boolean::Boolean::from_bool(!eql_other))
   }

   default_func!(BINARY: qt_gth, qt_gth_l, qt_gth_r);
   default_func!(BINARY: qt_lth, qt_lth_l, qt_lth_r);
   default_func!(BINARY: qt_leq, qt_leq_l, qt_leq_r);
   default_func!(BINARY: qt_geq, qt_geq_l, qt_geq_r);
   
   default_func!(BINARY: qt_cmp, qt_cmp_l, qt_cmp_r);
   default_func!(BINARY: qt_rgx, qt_rgx_l, qt_rgx_r);

   fn qt_get(&self, key: ObjRc, access_type: AccessType, env: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_set(&mut self, key: ObjRc, value: ObjRc, access_type: AccessType, env: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_call(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   // fn qt_get_l(&self, other: ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }
   // fn qt_get_r(&self, other: ObjRc, env: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }


}









