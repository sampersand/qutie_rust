use objects::object::Object;
use std::hash::Hash;
use std;

pub type BoxedObj = Box<Object>;



impl PartialEq for BoxedObj {
   fn eq(&self, other: &BoxedObj) -> bool {
      panic!("TODO: eq")
   }
}


impl Eq for BoxedObj {}

impl Hash for BoxedObj {
   fn hash<T>(&self, tpe: &mut T) {
      panic!("TODO: hash")
   }
}
