use objects::object::Object;
use objects::obj_rc::ObjRc;
use objects::types::Type;
use std::rc::Rc;

pub fn get_class<O: Object + ?Sized>(obj: &O) -> Rc<Type> {
   Rc::<Type>::from(obj.obj_type())
}

pub fn get_method<O: Object + ?Sized>(obj: &O, name: &str) -> Option<ObjRc> {
   match name {
      "__class" => Some(get_class(obj)),
      _ => None
   }
}
