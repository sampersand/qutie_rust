pub mod boxed_obj;
pub mod universe;
pub mod singlecharacter;
use std;

pub type Universe = universe::Universe;
pub type SingleCharacter = singlecharacter::SingleCharacter;
pub type BoxedObj = boxed_obj::BoxedObj;

use std::fmt::Debug;
pub trait Object : Debug {}

// use std::fmt::{Debug, Formatter, Error};

// impl Debug for Object{
//    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
//       write!(f, "Object{{ }}");
//       Ok(())
//    }
// }
