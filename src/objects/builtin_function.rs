use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{BoolResult, ObjError, ObjResult};

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use objects::universe::Universe;

pub struct BuiltinFunction {
   pub func: fn(Rc<&Universe>, &mut Environment) -> ObjResult,
}

impl BuiltinFunction {
   pub fn new(func: fn(Rc<&Universe>, &mut Environment) -> ObjResult) -> BuiltinFunction {
      BuiltinFunction{func: func}
   }
   pub fn to_string(&self) -> String {
      "<builtin_function>".to_string()
   }
}

impl Object for BuiltinFunction {
   impl_defaults!(OBJECT; BuiltinFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let args = match args.obj_type() {
         ObjType::Universe(uni) => uni,
         other @ _ => panic!("Cant call {:?} with type: {:?}", self, other)
      };
      (self.func)(rc!(args), env)
   }

   // obj_functions!(QT_EQL; BuiltinFunction, func);
}

impl_defaults!(DISPLAY_DEBUG; BuiltinFunction, 'F');


