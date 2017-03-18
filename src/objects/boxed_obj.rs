use objects::object::Object;
use std::hash::{Hash, Hasher};
use result::ObjError;

// pub type ObjBox = ObjBox;

impl PartialEq for ObjBox {
   fn eq(&self, other: &ObjBox) -> bool {
      match (**self).qt_eql(other) {
         Ok(obj) => obj.to_bool(),
         Err(ObjError::NotImplemented) => {println!("notimpl: {:?}, {:?}", self, other);false},
         Err(err) => panic!("Unexpected ObjError: {:?}", err)
      }
   }
}
impl Eq for ObjBox{}
impl Hash for ObjBox{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[1]);
      // (*self).hash(hasher)
   }
}