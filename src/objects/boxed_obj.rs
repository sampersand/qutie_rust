use objects::object::Object;
use std::hash::Hash;

pub type BoxedObj = Box<Object>;


impl PartialEq for BoxedObj {
   fn eq(&self, other: &BoxedObj) -> bool {
        // (&self).downcast::<Object>()// == other.downcast::<Object>()
        // ;false

        false
   }
}


impl Eq for BoxedObj {}

impl Hash for BoxedObj {
   fn hash<T>(&self, tpe: &mut T) {
        
   }
}

// use std::fmt::{Debug, Formatter, Error};

// impl Debug for BoxedObj{
//    pub fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
//       write!(f, "BoxedObj{{ ? }}");
//       Ok(())
//    }
// }



