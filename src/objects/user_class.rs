use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, OldObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::boolean::Boolean;
use objects::universe::{Universe, AccessType};

pub struct UserClass {
   parents: Rc<Object>,
   body: Rc<Object>
}

impl UserClass {
   pub fn new(parents: Rc<Object>, body: Rc<Object>) -> UserClass {
      cast_as!(parents, Universe);
      cast_as!(body, Universe);
      UserClass{parents: parents, body: body}
   }
   pub fn to_string(&self) -> String {
      "<user_class>".to_string()
   }
}

impl Object for UserClass {
   impl_defaults!(OBJECT; UserClass);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      
      cast_as!(self.body, Universe).call(args, env, false)
   }

   // obj_functions!(QT_EQL; UserClass, func);
}

impl_defaults!(DISPLAY_DEBUG; UserClass, 'f');







