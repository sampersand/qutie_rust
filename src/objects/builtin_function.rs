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
pub struct BuiltinFunction {
   id: IdType,
   func: fn(Rc<Universe>, &mut Environment) -> ObjResult,
}

impl BuiltinFunction {
   pub fn new(func: fn(Rc<Universe>, &mut Environment) -> ObjResult) -> BuiltinFunction {
      BuiltinFunction{id: next_id!(), func: func}
   }
   pub fn to_rc(self) -> Rc<BuiltinFunction> {
      Rc::new(self)
   }
   pub fn to_string(&self) -> String {
      "<builtin_function>".to_string()
   }
}

impl Object for BuiltinFunction {
   impl_defaults!(OBJECT; BuiltinFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      (self.func)(cast_as!(args, Universe), env)
   }

   // obj_functions!(QT_EQL; func);
}

impl_defaults!(DISPLAY_DEBUG; BuiltinFunction, 'F');



