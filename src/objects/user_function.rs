use globals::IdType;
use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use std::cell::RefCell;
use result::{ObjError, ObjResult, BoolResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::symbol::Symbol;
use objects::boolean::{Boolean, BoolType};
use objects::universe::{Universe, AccessType};

pub struct UserFunction {
   id: IdType,
   args: Rc<Universe>,
   body: Rc<Universe>,
   parent: Option<Rc<Universe>>,
}

impl UserFunction {
   pub fn new(args: Rc<Universe>, body: Rc<Universe>) -> UserFunction {
      UserFunction{id: next_id!(), args: args, body: body, parent: None }
   }
   pub fn to_rc(self) -> Rc<UserFunction> {
      Rc::new(self)
   }
   pub fn to_string(&self) -> String {
      "<user_function>".to_string()
   }
   // pub fn is_method(&self) -> bool {
   //    let ref stack = self.args.stack;
   //    1 <= stack.len() && cast_as!(CL; stack.get(0).unwrap(), Symbol).sym_val.as_str() == "__self"
   // }

   pub fn set_parent(&self, parent: Rc<Universe>) {
      unsafe { // this works for the current bug
         use std::mem::transmute;
         #[allow(mutable_transmutes)]
         let tmp = transmute::<&UserFunction, &mut UserFunction>(self);
         tmp.parent = Some(parent); // tmp needed because the allow mutable_transmutes statement won't let me do assignment
      }
   }
   pub fn get_parent(&self) -> ObjRc {
      self.parent.clone().expect("Can't get parent!").clone()
   }
}

impl Object for UserFunction {
   impl_defaults!(OBJECT; UserFunction);
   obj_functions!(QT_TO_TEXT);
   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let args_clone = args.clone();
      let args_uni = cast_as!(args, Universe);
      let mut call_args = unsafe { // works for current bug
         use std::mem;
         #[allow(mutable_transmutes)] 
         mem::transmute::<&Universe, &mut Universe>(&*args_uni)
      };

      /* set __self to the current parent. */
      call_args.set(new_obj!(SYM_STATIC, "__self"), 
                     if let Some(parent) = self.parent.clone(){
                        parent
                     } else {
                        new_obj!(BOOL_STATIC, Null)
                     }, AccessType::Locals);

      /* Update each element */
      for (pos, ele) in args_uni.stack.iter().enumerate() {
         let key = self.args.stack.get(pos).expect("Position (??) isnt defined");

         if args_uni.locals.contains_key(&ObjRcWrapper(key.clone())) {
            panic!("position `{:?}` is also given as a keyword argument", pos);
         } else {
            call_args.set(key.clone(), ele.clone(), AccessType::Locals);
         }
      }
      self.body.qt_call(args_uni, env)
   }

   // obj_functions!(QT_EQL; func);
}

impl_defaults!(DISPLAY_DEBUG; UserFunction, 'f');







