use globals::IdType;
use env::Environment;

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::text::Text;
use std::rc::Rc;
use objects::boolean::Boolean;
use objects::obj_rc::ObjRc;
use result::{ObjError, BoolResult};

#[derive(Clone)]
pub struct Type {
   id: IdType,
   obj_type: ObjType,
}

const BOOLEAN: Type = Type{ id: next_id!(STATIC), obj_type: ObjType::Boolean };
const TEXT: Type = Type{ id: next_id!(STATIC), obj_type: ObjType::Text };
const NUMBER: Type = Type{ id: next_id!(STATIC), obj_type: ObjType::Number };

impl Type {
   pub fn to_string(&self) -> String {
      "<type>".to_string()
      // self.obj_type.to_string()
   }
   pub fn to_rc(self) -> Rc<Type> {
      Rc::new(self)
   }
}

impl From<ObjType> for Rc<Type> {
   fn from(inp: ObjType) -> Rc<Type> {
      match inp {
         ObjType::Boolean => BOOLEAN.to_rc(),
         ObjType::Text => TEXT.to_rc(),
         ObjType::Number => NUMBER.to_rc(),
         typ @ _ => panic!("bad objtype for Type: {:?}", typ)
      }
   }
}

impl Object for Type {
   impl_defaults!(OBJECT; Type);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; obj_type);
}

impl_defaults!(DISPLAY_DEBUG; Type, 't');












