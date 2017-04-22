use globals::IdType;
use env::Environment;
use objects::text::Text;
use std::rc::Rc;
use result::{ObjError, BoolResult};

use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;

pub struct Symbol<'a> {
   // symbol needs an id
   id: IdType,
   pub sym_val: &'a str,
}

impl <'a> Symbol<'a> {
   pub fn new(inp: &'a str) -> Symbol<'a> {
      Symbol{id: next_id!(), sym_val: inp}
   }
   pub fn to_rc(self) -> Rc<Symbol<'a>> {
      Rc::new(self)
   }

   pub fn to_string(&self) -> String {
      self.sym_val.to_string()
   }
}

unsafe fn to_static<'a>(inp: String) -> &'static str {
   use std::mem;
   let res = mem::transmute(&inp as &str);
   mem::forget(inp);
   res
}

impl <'a> From<String> for Symbol<'a> {
   fn from(inp: String) -> Symbol<'static> {
      unsafe {
         Symbol::new(to_static(inp))
      }
   }
}

impl <'a> Object for Symbol<'a> {
   impl_defaults!(OBJECT; Symbol);
   obj_functions!(QT_TO_TEXT);
   obj_functions!(QT_EQL; sym_val);
}

use std::fmt::{Debug, Formatter, Error, Display};
impl <'a> Display for Symbol<'a> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.to_string())
   }
}

impl <'a> Debug for Symbol<'a> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "S({}|{})", self, self.id)
   }
}


