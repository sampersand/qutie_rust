use std::fmt::{Debug, Display};
use std::rc::Rc;
use objects::number::Number;
use objects::boolean::{Boolean, BoolType};
use objects::text::Text;
use objects::single_character::SingleCharacter;
use objects::universe::AccessType;
use objects::obj_rc::ObjRc;
use result::{ObjResult, ObjError, BoolResult};
use env::Environment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjWrapper<T: Object>(pub Rc<T>);

impl <T: Object> ObjWrapper<T> {
   unsafe fn _unsafe_from(obj: Rc<Object>) -> ObjWrapper<T> {
      let obj = obj.clone();
      use std::mem::transmute;
      ObjWrapper(transmute::<&Rc<Object>, &Rc<T>>(&obj).clone())
   }
}
impl <T: Object> From<ObjRc> for ObjWrapper<T> {
   fn from(obj: ObjRc) -> ObjWrapper<T> {
      unsafe { ObjWrapper::_unsafe_from(obj) }
   }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjType {
   Universe,
   Number,
   SingleCharacter,
   Symbol,
   Text,
   Boolean,
   Operator,
   BuiltinFunction,
   /*BuiltinMethod,*/
   UserFunction,
   UserClass,
   Builtin,
   User
}

macro_rules! default_func {
   (UNARY: $name:ident, $ret_type:ty) => {
      fn $name(&self, _: &mut Environment) -> $ret_type { Err(ObjError::NotImplemented) }
   };
   (BINARY: $name:ident, $name_l:ident, $name_r:ident, $_type:ty) => {
      fn $name(&self, other: ObjRc, env: &mut Environment) -> Result<Rc<$_type>, ObjError> {
         match self.$name_l(other.clone(), env) {
            // Err(ObjError::NotImplemented) => other.$name_r(self.get_rc().unwrap(), env),
            other @ _ => other
         }
      }
   };

   (BINARY_ALL: $name:ident, $name_l:ident, $name_r:ident, $_type:ty) => {
      default_func!(BINARY: $name, $name_l, $name_r, $_type);
      fn $name_l(&self, _: ObjRc, _: &mut Environment) -> Result<Rc<$_type>, ObjError> {
         Err(ObjError::NotImplemented)
      }
      fn $name_r(&self, _: ObjRc, _: &mut Environment) -> Result<Rc<$_type>, ObjError> {
         Err(ObjError::NotImplemented)
      }
   };
}

pub trait Object : Debug + Display {
   fn is_a(&self, obj_type: ObjType) -> bool { self.obj_type() == obj_type }
   fn obj_type(&self) -> ObjType;
   fn source(&self) -> Vec<SingleCharacter>;
   fn get_rc(&self) -> Option<ObjRc> {
      None
   }
   fn _eql(&self, other: ObjRc, env: &mut Environment) -> bool {
      if let Ok(obj) = self.qt_eql(other, env) {
         obj.bool_val
      } else {
         false
      }
   }
   default_func!(UNARY: qt_to_bool, Result<Rc<Boolean>, ObjError>);
   default_func!(UNARY: qt_to_num, Result<Rc<Number>, ObjError>);
   default_func!(UNARY: qt_to_text, Result<Rc<Text>, ObjError>);

   fn qt_exec(&self, _: &mut Environment) -> ObjResult { Err(ObjError::NotImplemented) }

   default_func!(BINARY_ALL: qt_add, qt_add_l, qt_add_r, Object);
   default_func!(BINARY_ALL: qt_sub, qt_sub_l, qt_sub_r, Object);
   default_func!(BINARY_ALL: qt_mul, qt_mul_l, qt_mul_r, Object);
   default_func!(BINARY_ALL: qt_div, qt_div_l, qt_div_r, Object);
   default_func!(BINARY_ALL: qt_mod, qt_mod_l, qt_mod_r, Object);
   default_func!(BINARY_ALL: qt_pow, qt_pow_l, qt_pow_r, Object);
   default_func!(BINARY: qt_eql, qt_eql_l, qt_eql_r, Boolean);

   fn qt_eql_l(&self, _: ObjRc, _: &mut Environment) -> BoolResult {
      Err(ObjError::NotImplemented)
   }
   fn qt_eql_r(&self, _: ObjRc, _: &mut Environment) -> BoolResult {
      Ok(new_obj!(BOOL_STATIC, False))
   }

   default_func!(BINARY: qt_neq, qt_neq_l, qt_neq_r, Boolean);

   fn qt_neq_l(&self, other: ObjRc, env: &mut Environment) -> BoolResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_neq_r(&self, other: ObjRc, env: &mut Environment) -> BoolResult {
      Ok(Boolean::from(!self._eql(other, env)).to_rc())
   }

   default_func!(BINARY_ALL: qt_gth, qt_gth_l, qt_gth_r, Boolean);
   default_func!(BINARY_ALL: qt_lth, qt_lth_l, qt_lth_r, Boolean);
   default_func!(BINARY_ALL: qt_leq, qt_leq_l, qt_leq_r, Boolean);
   default_func!(BINARY_ALL: qt_geq, qt_geq_l, qt_geq_r, Boolean);
   
   default_func!(BINARY_ALL: qt_cmp, qt_cmp_l, qt_cmp_r, Boolean);
   default_func!(BINARY_ALL: qt_rgx, qt_rgx_l, qt_rgx_r, Object);

   fn qt_get(&self, key: ObjRc, env: &mut Environment) -> ObjResult {
      match self.qt_get_l(key.clone(), env) {
         Ok(obj) => Ok(obj),
         Err(ObjError::NoSuchKey(_)) =>
            if key.is_a(ObjType::Text) {
               self.qt_method(to_type!(STRING; key, env).as_str(), env)
            } else {
               Err(ObjError::NotImplemented)      
            },
         Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
         Err(err) => unreachable!("Bad error in qt_get: {:?}", err)
      }
   }

   fn qt_get_l(&self, _: ObjRc, _: &mut Environment) -> ObjResult{
      Err(ObjError::NotImplemented)
   }

   fn qt_set(&mut self, _: ObjRc, _: ObjRc, _: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_call(&self, _: ObjRc, _: &mut Environment) -> ObjResult {
      Err(ObjError::NotImplemented)
   }

   fn qt_method(&self, other: &str, _: &mut Environment) -> ObjResult {
      if other == "__class" {

      } else {
         Err(ObjError::NoSuchKey(new_obj!(TEXT, other.to_string())))
      }
   }

}
















