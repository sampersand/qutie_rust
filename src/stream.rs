use objects::obj_rc::ObjRc;
use std::rc::Rc;
use objects::single_character::SingleCharacter;

pub type StreamChar = char;
type StreamType = Vec<StreamChar>;

#[derive(Debug)]
pub struct StreamCharWrapper<'a>{
   pub chr: StreamChar,
   stream: &'a mut Stream
}

impl <'a> StreamCharWrapper<'a> {
   pub fn take(&mut self) -> StreamChar {
      assert_debug!(eq; self.chr, self.stream.next().expect("EOF whilst removing"));
      self.chr
   }
}

use std::ops::Deref;
impl <'a> Deref for StreamCharWrapper<'a> {
   type Target = StreamChar;
   fn deref(&self) -> &StreamChar {
      &self.chr
   }
}

#[derive(Debug, Clone)]
pub struct Stream {
   source: StreamType
}

impl Stream {
   pub fn new(inp: StreamType) -> Stream {
      Stream {
         source: inp 
      }
   }
   pub fn to_raw_string(&self) -> String {
      let mut ret = String::new();
      for chr in self.source.clone() {
         ret.push(chr)
      }
      ret
   }
   pub fn is_empty(&self) -> bool {
      self.source.is_empty()
   }
   pub fn from_str(inp: &str) -> Stream {
      use std::iter::FromIterator;
      Stream::new(Vec::<StreamChar>::from_iter(inp.chars()))
   }

   pub fn feed(&mut self, other: StreamChar) {
      self.source.insert(0, other);
   }

   pub fn feed_back(&mut self, other: ObjRc) {
      let mut src = other.source();
      src.reverse();
      for x in src {
         self.feed(x.char_val);
      }
   }

   pub fn next(&mut self) -> Option<StreamChar> {
      if self.is_empty() {
         None 
      } else {
         Some(self.source.remove(0))
      }
   }

   pub fn next_single_char(&mut self) -> Option<Rc<SingleCharacter>> {
      if let Some(chr) = self.next() {
         Some(SingleCharacter::new(chr).to_rc())
      } else {
         None
      }
   }
   pub fn peek(&mut self) -> Option<StreamCharWrapper> {
      let chr = match self.source.first() {
         Some(chr) => chr.clone(),
         None => return None
      };
      Some(StreamCharWrapper{chr: chr, stream: self})
   }
}


















