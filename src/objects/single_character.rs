use objects::object::{Object, ObjType};

type SourceType = char;

#[derive(Eq, PartialEq, Clone)]
pub struct SingleCharacter {
   pub char_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{char_val: inp}
   }
   pub fn to_string(&self) -> String {
      self.char_val.to_string()
   }
}

impl Object for SingleCharacter {
   impl_defaults!(OBJECT; SingleCharacter);
}
impl_defaults!(DISPLAY_DEBUG; SingleCharacter, 'C');
