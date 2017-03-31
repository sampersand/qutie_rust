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
   args: Rc<Object>,
   body: Rc<Object>
}

impl UserFunction {
   pub fn new(args: Rc<Object>, body: Rc<Object>) -> UserFunction {
      cast_as!(args, Universe);
      cast_as!(body, Universe);
      UserFunction{args: args, body: body}
   }
   pub fn to_string(&self) -> String {
      "<user_function>".to_string()
   }
}

impl Object for UserFunction {
   impl_defaults!(OBJECT; UserFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let self_args = cast_as!(self.args, Universe);
      let args_clone = args.clone();
      let uni_args = cast_as!(args_clone, Universe);
      let mut call_args = unsafe {
         use std::mem;
         #[allow(mutable_transmutes)] 
         mem::transmute::<&Universe, &mut Universe>(uni_args)
      };

      let ref self_stack = self_args.stack;
      let ref stack = uni_args.stack;
      let ref locals = uni_args.locals;

      for pos in 0..stack.len() {
         let ele = stack.get(pos).unwrap();
         if locals.contains_key(&rc_wrap!(ele.clone())) {
            panic!("position {:?} is also given as a keyword argument", pos);
         } else {
            call_args.set(self_stack.get(pos).expect("position isnt defined").clone(),
                          ele.clone(),
                          AccessType::Locals);
         }
      }

      self.body.qt_call(args, env)
   }

   // obj_functions!(QT_EQL; UserFunction, func);
}

impl_defaults!(DISPLAY_DEBUG; UserFunction, 'f');







