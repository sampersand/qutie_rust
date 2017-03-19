use globals;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::ops::Deref;
use objects::object::Object;
use result::ObjError;

use std::fmt::{Display, Formatter, Error, Debug};

pub type ObjRc = Rc<Object>;

#[derive(Clone)]
pub struct ObjRcWrapper(pub ObjRc);

impl Display for ObjRcWrapper {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.0)
   }
}
impl Debug for ObjRcWrapper {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{:?}", self.0)
   }
}

impl PartialEq for ObjRcWrapper {
   fn eq(&self, other: &ObjRcWrapper) -> bool {
      let env = unsafe {
        &mut *globals::GLOBAL_ENV
      };

      match (*self.0).qt_eql(&other.0, env) {
         Ok(obj) => obj.to_bool(),
         Err(ObjError::NotImplemented) => false,
         Err(err) => panic!("TODO: impl {:?}", err)
      }
   }
}
impl Eq for ObjRcWrapper{}
impl Hash for ObjRcWrapper{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[1]);

      // (*self).hash(hasher)
   }
}





