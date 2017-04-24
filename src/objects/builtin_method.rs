use globals::IdType;
use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::universe::Universe;

#[allow(dead_code)]
pub struct BuiltinMethod<T: Object> {
   id: IdType,
   obj: Rc<T>,
   func: fn(Rc<T>, Rc<Universe>, &mut Environment) -> ObjResult,
}

impl <T: Object> BuiltinMethod<T> {
   pub fn new(obj: Rc<T>,
              func: fn(Rc<T>, Rc<Universe>, &mut Environment) -> ObjResult) -> BuiltinMethod<T> {
      BuiltinMethod{id: next_id!(), obj: obj, func: func}
   }
   pub fn to_rc(self) -> Rc<BuiltinMethod<T>> {
      Rc::new(self)
   }
   pub fn to_string(&self) -> String {
      "<builtin_method>".to_string()
   }
}

impl <T: Object> Object for BuiltinMethod<T> {
   impl_defaults!(OBJECT; BuiltinMethod);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      (self.func)(self.obj.clone(), cast_as!(args, Universe), env)
   }

   // obj_functions!(QT_EQL; func);
}

use std::fmt::{Debug, Formatter, Error, Display};
impl <T: Object> Display for BuiltinMethod<T> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.to_string())
   }
}

impl <T: Object> Debug for BuiltinMethod<T> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "Bm({})", self)
   }
}
