use objects::Object;

pub type BoxObj = Box<Object>;

use std;

impl PartialEq for BoxObj {
    fn eq(&self, other: &BoxObj) -> bool {
        // (&self).downcast::<Object>()// == other.downcast::<Object>()
        ;false
    }
}

impl Eq for BoxObj {}

impl std::hash::Hash for BoxObj {
    fn hash<T>(&self, tpe: &mut T){
        
    }
}

// use std::fmt::{Debug, Formatter, Error};

// impl Debug for BoxObj{
//    pub fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
//       write!(f, "BoxObj{{ ? }}");
//       Ok(())
//    }
// }



