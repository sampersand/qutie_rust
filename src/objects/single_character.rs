use globals::IdType;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use objects::text::Text;
use std::rc::Rc;
use env::Environment;
use result::{ObjError, BoolResult};
use objects::object::{Object, ObjType, ObjWrapper};

type SourceType = char;

#[derive(Eq, PartialEq, Clone)]
pub struct SingleCharacter {
   id: IdType,
   pub char_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{id: next_id!(), char_val: inp}
   }
   pub fn to_rc(self) -> Rc<SingleCharacter> {
      Rc::new(self)
   }
}

impl Object for SingleCharacter {
   impl_defaults!(OBJECT; SingleCharacter);
   obj_functions!(QT_EQL; char_val);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(TO_STRING; char_val);

}
impl_defaults!(DISPLAY_DEBUG; SingleCharacter, 'C');
