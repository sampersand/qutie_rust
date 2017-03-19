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
}

impl Object for SingleCharacter{
   fn obj_type(&self) -> ObjType{ ObjType::SingleCharacter(self) }
   fn source(&self) -> Vec<SingleCharacter> { vec![self.clone()] }
}

display_debug!(SingleCharacter, 'C', char_val);
