use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, ObjResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::boolean::{Boolean, BoolType};
use objects::symbol::Symbol;
use objects::universe::{Universe, AccessType};

static mut GID: u32 = 0;
pub struct UserClass {
   id: u32,
   parents: Rc<Universe>,
   body: Rc<Universe>,
   rc: Option<Rc<UserClass>>
}

impl UserClass {
   pub fn new(parents: Rc<Universe>, body: Rc<Universe>) -> UserClass {
      UserClass{id: unsafe{GID = GID + 1; GID}, parents: parents, body: body, rc: None}
   }
   pub fn to_rc(mut self) -> Rc<UserClass> {
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
      let ret = self.body.call(cast_as!(args, Universe), env, false).unwrap();
      assert_debug!(ret.is_a(ObjType::Universe), "ret isnt a universe in UserClass");
      let ret = cast_as!(ret, Universe);
      let mut uni = Rc::try_unwrap(ret).unwrap();
      uni.set(new_obj!(SYM_STATIC, "__class"),
              self.rc.clone().unwrap(),
              AccessType::Locals);
      
      // let mut uni: &mut Universe = unsafe {
      //    use std::mem::transmute;
      //    #[allow(mutable_transmutes)]
      //    transmute(&&*ret)
      // };
      // println!("self.rca: {:?}", self.rc.clone());
      Ok(uni.to_rc())
   }
   fn qt_eql_l(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
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



















