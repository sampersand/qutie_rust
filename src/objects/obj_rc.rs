use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::ops::Deref;
use objects::object::Object;
use result::ObjError;

use std::fmt::{Display, Formatter, Error, Debug};

// #[derive()]
pub struct ObjRc(Rc<Object>);

impl Display for ObjRc {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.num_val)
   }
}
impl Debug for ObjRc {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}


impl PartialEq for ObjRc {
   fn eq(&self, other: &ObjRc) -> bool {
      match (**self).qt_eql(other) {
         Ok(obj) => obj.to_bool(),
         Err(ObjError::NotImplemented) => {println!("notimpl: {:?}, {:?}", self, other);false},
         Err(err) => panic!("Unexpected ObjError: {:?}", err)
      }
   }
}
impl Deref for ObjRc {
   type Target = Rc<Object>;
   fn deref(&self) -> &Rc<Object> {
      &self.0
   }
}

impl Eq for ObjRc{}
impl Hash for ObjRc{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[1]);
      // (*self).hash(hasher)
   }
}