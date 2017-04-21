use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::boolean::Boolean;
use objects::symbol::Symbol;
use objects::universe::{Universe, AccessType};

pub struct UserClass {
   parents: Rc<Universe>,
   body: Rc<Universe>
}

impl UserClass {
   pub fn new(parents: Rc<Universe>, body: Rc<Universe>) -> UserClass {
      UserClass{parents: parents, body: body}
   }
   pub fn to_rc(self) -> Rc<UserClass> {
      Rc::new(self)
   }
   pub fn to_string(&self) -> String {
      "<user_class>".to_string()
   }
}

impl Object for UserClass {
   impl_defaults!(OBJECT; UserClass);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let ret = self.body.call(cast_as!(args, Universe), env, false);
      cast_as!(ret.unwrap(), Universe).set(new_obj!(SYM_STATIC, "__class"), self, env);
      ret
   }
}
impl_defaults!(DISPLAY_DEBUG; UserClass, 'f');







