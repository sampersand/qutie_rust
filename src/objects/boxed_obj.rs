use objects::object::Object;
use std::hash::{Hash, Hasher};
use result::ObjError;

pub type BoxedObj = Box<Object>;

impl PartialEq for BoxedObj {
   fn eq(&self, other: &BoxedObj) -> bool {
      match (**self).qt_eql(other) {
         Ok(obj) => obj.to_bool(),
         Err(ObjError::NotImplemented) => {println!("notimpl: {:?}, {:?}", self, other);false},
         Err(err) => panic!("Unexpected ObjError: {:?}", err)
      }
   }
}
impl Eq for BoxedObj{}
impl Hash for BoxedObj{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[1]);
      // (*self).hash(hasher)
   }
}