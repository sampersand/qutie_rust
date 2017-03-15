use objects::object::Object;
use std::hash::{Hash, Hasher};
use std;

pub type BoxedObj = Box<Object>;



impl PartialEq for BoxedObj {
   fn eq(&self, other: &BoxedObj) -> bool {
      self as *const BoxedObj == other as *const BoxedObj
   }
}


impl Eq for BoxedObj {}

impl Hash for BoxedObj {
    fn hash<H: Hasher>(&self, state: &mut H) {
      (self as *const BoxedObj).hash(state)
    }
}
