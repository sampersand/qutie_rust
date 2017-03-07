use objects::Object;

type SourceType = char;

pub struct SingleCharacter {
   pub source_val: SourceType
}

impl SingleCharacter {
   pub fn new(inp: SourceType) -> SingleCharacter {
      SingleCharacter{source_val: inp}
   }
}

impl Object for SingleCharacter{}

use std::fmt::{Debug, Formatter, Error};

impl Debug for SingleCharacter{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f,
             "SingleCharacter{{ {} }}",
             match self.source_val{
               ' ' => "_".to_string(),
               '_' => "\\_".to_string(),
               e @ _ => e.to_string()
             });
      Ok( () )
   }
   // pub fn
}