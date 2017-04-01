use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use objects::universe::Universe;

pub struct BuiltinMethod<'a> {
   instance: &'a Object,
   func: fn(&'a Object, Rc<&Universe>, &mut Environment) -> ObjResult,
}

impl <'a> BuiltinMethod<'a> {
   pub fn new(instance: &'a Object,
              func: fn(&'a Object, Rc<&Universe>, &mut Environment) -> ObjResult) -> BuiltinMethod<'a> {
      BuiltinMethod{instance: instance, func: func}
   }
   pub fn to_string(&self) -> String {
      "<builtin_method of ".to_string() + self.instance.to_string().as_str() + ">"
   }
}

impl <'a> Object for BuiltinMethod<'a> {
   fn obj_type(&self) -> ObjType {
      // ObjType::BuiltinMethod<'a>(self)
      panic!("TODO: ObjType")
   }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      (self.func)(self.instance, rc!(cast_as!(args, Universe)), env)
   }

   // obj_functions!(QT_EQL; BuiltinMethod, func);
}

use std::fmt::{Debug, Formatter, Error, Display};
impl <'a> Display for BuiltinMethod<'a> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.to_string())
   }
}

impl <'a> Debug for BuiltinMethod<'a> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}({})", 'M', self)
   }
}





