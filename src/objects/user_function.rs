use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::boolean::Boolean;
use objects::universe::{Universe, AccessType};

pub struct UserFunction {
   args: ObjRc,
   body: ObjRc,
   pbu parent: ObjRc,
}

impl UserFunction {
   pub fn new(args: ObjRc, body: ObjRc, parent: ObjRc) -> UserFunction {
      cast_as!(args, Universe);
      cast_as!(body, Universe);
      UserFunction{args: args, body: body, parent: ObjRc}
   }
   pub fn to_string(&self) -> String {
      "<user_function>".to_string()
   }
   pub fn is_method(&self) -> bool {
      true
   }
   pub fn get_parent(&self) -> ObjRc {
      self.parent
   }
}

impl Object for UserFunction {
   impl_defaults!(OBJECT; UserFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let self_args = cast_as!(self.args, Universe);
      let args_clone = args.clone();
      let args_uni = cast_as!(args_clone, Universe);
      let mut call_args = unsafe {
         use std::mem;
         #[allow(mutable_transmutes)] 
         mem::transmute::<&Universe, &mut Universe>(args_uni)
      };

      let ref self_stack = self_args.stack;
      let ref stack = args_uni.stack;
      let ref locals = args_uni.locals;

      for pos in 0..stack.len() {
         let ele = stack.get(pos).unwrap();
         let key = self_stack.get(pos).expect("position isnt defined");
         if locals.contains_key(&rc_wrap!(key.clone())) {
            panic!("position `{:?}` is also given as a keyword argument", pos);
         } else {
            call_args.set(key.clone(),
                          ele.clone(),
                          AccessType::Locals);
         }
      }
      self.body.qt_call(args, env)
   }

   // obj_functions!(QT_EQL; UserFunction, func);
}

impl_defaults!(DISPLAY_DEBUG; UserFunction, 'f');







