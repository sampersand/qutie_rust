use objects::object::Object;
use objects::obj_rc::ObjRc;
use objects::text::Text;
use objects::universe::Universe;
use objects::number::{Number, NumberType};
use result::ObjError;
use std::rc::Rc;
use env::Environment;

pub fn get_length(obj: Rc<Text>, arg: Rc<Universe>, _: &mut Environment) -> Result<Rc<dyn Object>, ObjError> {
   Ok(new_obj!(NUM, obj.text_val.len() as NumberType))
}

pub fn get_method(obj: &Text, name: &str) -> Option<ObjRc> {
   match name {
      "length" => Some(rc_meth!(obj, Text, get_length)),
      o @ _ => panic!("bad name: {:?}", name)
   }
}
