use objects::object::{Object, ObjType, ObjWrapper};
use objects::obj_rc::ObjRc;
use objects::text::Text;

macro_rules! rc_meth {
   ($obj:expr, $T:ty, $func:path) => {{
      use objects::builtin_method::BuiltinMethod; 
      BuiltinMethod::<$T>::new($obj.get_rc().expect("Cant get BuiltinMethod"), $func).to_rc()
   }} 
}

mod object_methods;
mod text_methods;

pub fn get_method<O: Object + ?Sized>(obj: &O, name: &str) -> Option<ObjRc> {
   if let Some(meth) = object_methods::get_method(obj, name) {
      return Some(meth);
   }
   macro_rules! cast_to {
       ($from:expr, $to:ident) => {
         unsafe{
            use std::mem::transmute;
            *transmute::<&&O, &&$to>(&$from)
         }
       }
   }
   match obj.obj_type() {
      ObjType::Text => text_methods::get_method(cast_to!(obj, Text), name),
      other_type @ _ => panic!("Unimplemented method type: {:?}", other_type)
   }

}










