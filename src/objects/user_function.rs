use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use std::cell::RefCell;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::symbol::Symbol;
use objects::boolean::Boolean;
use objects::universe::{Universe, AccessType};

pub struct UserFunction {
   args: Rc<Universe>,
   body: Rc<Universe>,
   parent: Option<Rc<Universe>>,
}

impl UserFunction {
   pub fn new(args: Rc<Universe>, body: Rc<Universe>) -> UserFunction {
      UserFunction{args: args, body: body, parent: None }
   }
   pub fn to_string(&self) -> String {
      "<user_function>".to_string()
   }
   pub fn is_method(&self) -> bool {
      let ref stack = self.args.stack;
      1 <= stack.len() && cast_as!(CL; stack.get(0).unwrap(), Symbol).sym_val.as_str() == "__self"
   }
   pub fn set_parent(&self, parent: Rc<Universe>) {
      unsafe {
         use std::mem::transmute;
         #[allow(mutable_transmutes)]
         let tmp = transmute::<&UserFunction, &mut UserFunction>(self);
         tmp.parent = Some(parent);
      }
   }
   pub fn get_parent(&self) -> ObjRc {
      use std::ops::Deref;
      match self.parent.clone() {
         Some(obj) => obj.clone(),
         None => panic!("CANT UNWRAP")
      }
   }
}

impl Object for UserFunction {
   impl_defaults!(OBJECT; UserFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let args_clone = args.clone();
      let args_uni = cast_as!(args_clone, Universe);
      let mut call_args = unsafe {
         use std::mem;
         #[allow(mutable_transmutes)] 
         mem::transmute::<&Universe, &mut Universe>(&*args_uni)
      };

      let ref self_stack = self.args.stack;
      let ref stack = args_uni.stack;
      let ref locals = args_uni.locals;
      let mut self_pos = 0;
      if self.is_method() {
         call_args.set(rc!(Symbol::from("__self")), self.parent.clone().unwrap(), AccessType::Locals);
         self_pos += 1;
      }
      for pos in 0..stack.len() {
         let ele = stack.get(pos).unwrap();
         let key = self_stack.get(self_pos).expect("position isnt defined");
         if locals.contains_key(&rc_wrap!(key.clone())) {
            panic!("position `{:?}` is also given as a keyword argument", pos);
         } else {
            call_args.set(key.clone(),
                          ele.clone(),
                          AccessType::Locals);
         }
         self_pos += 1;
      }
      self.body.qt_call(args, env)
   }

   // obj_functions!(QT_EQL; func);
}

impl_defaults!(DISPLAY_DEBUG; UserFunction, 'f');







