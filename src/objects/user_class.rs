use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult, BoolResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::{Boolean, BoolType};
use objects::symbol::Symbol;
use objects::universe::{Universe, AccessType};
use globals::IdType;

#[allow(dead_code)]
pub struct UserClass {
   id: IdType,
   parents: Rc<Universe>,
   body: Rc<Universe>,
   rc: Option<Rc<UserClass>>
}

impl UserClass {
   pub fn new(parents: Rc<Universe>, body: Rc<Universe>) -> UserClass {
      UserClass{ id: next_id!(),
                 parents: parents,
                 body: body,
                 rc: None }
   }
   pub fn to_rc(self) -> Rc<UserClass> {
      let ret = Rc::new(self);
      unsafe {
         use std::mem::transmute;
         #[allow(mutable_transmutes)]
         transmute::<&UserClass, &mut UserClass>(&*ret)
      }.rc = Some(ret.clone());
      ret
   }
   pub fn to_string(&self) -> String {
      "<user_class>".to_string()
   }
}

impl Object for UserClass {
   impl_defaults!(OBJECT; UserClass);
   obj_functions!(QT_TO_TEXT);


   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      let ret = self.body.call(cast_as!(args, Universe), env, false).expect("err when calling body");
      assert_debug!(ret.is_a(ObjType::Universe), "ret isnt a universe in UserClass");
      let ret = cast_as!(ret, Universe);
      let mut uni = Rc::try_unwrap(ret).expect("error when unwrapping ret, user_class/qt_call");
      uni.set(new_obj!(SYM_STATIC, "__class"),
              self.rc.clone().expect("error self.rc.clone()"),
              AccessType::Locals);
      
      // let mut uni: &mut Universe = unsafe {
      //    use std::mem::transmute;
      //    #[allow(mutable_transmutes)]
      //    transmute(&&*ret)
      // };
      // println!("self.rca: {:?}", self.rc.clone());
      Ok(uni.to_rc())
   }
   fn qt_eql_l(&self, other: ObjRc, _: &mut Environment) -> BoolResult {
      if !other.is_a(ObjType::UserClass){
         return Ok(new_obj!(BOOL_STATIC, False));
      }
      let other = cast_as!(other, UserClass);
      // println!("self.parents: {:?}, other.parents: {:?}", self.parents, other.parents);
      // println!("parents: {:?}", self.parents._eql(other.parents.clone(), env));
      Ok(new_obj!(BOOL, self.id == other.id))
                  // self.parents._eql(other.parents.clone(), env) && 
                  // self.body._eql(other.body.clone(), env)))
   }
}
impl_defaults!(DISPLAY_DEBUG; UserClass, 'f');



















