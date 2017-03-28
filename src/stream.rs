use objects::obj_rc::ObjRc;
use std::rc::Rc;
use objects::object::Object;
use objects::single_character::SingleCharacter;

type StreamType = Vec<char>;
#[derive(Debug)]
pub struct Stream {
   source: StreamType
}

impl Stream {
   pub fn new(inp: StreamType) -> Stream {
      Stream {
         source: inp 
      }
   }
   pub fn is_empty(&self) -> bool {
      self.source.is_empty()
   }
   pub fn from_str(inp: &str) -> Stream {
      use std::iter::FromIterator;
      Stream::new(Vec::<char>::from_iter(inp.chars()))
   }

   pub fn feed(&mut self, other: char) {
      self.source.insert(0, other);
   }

   pub fn feed_back(&mut self, other: ObjRc) {
      let mut src = other.source();
      src.reverse();
      for x in src {
         self.feed(x.char_val);
      }
   }

   pub fn next(&mut self) -> Option<char> {
      if self.source.len() == 0 {
         None 
      } else {
         Some(self.source.remove(0))
      }
   }
   pub fn next_single_char(&mut self) -> Option<Rc<SingleCharacter>> {
      if let Some(chr) = self.next() {
         Some(rc!(SingleCharacter::new(chr)))
      } else {
         None
      }
   }

   pub fn peek(&self) -> Option<&char> { // aka ObjResult w/ a reference
      self.source.first()
   }
}







