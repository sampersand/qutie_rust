pub mod boxed_obj;
pub mod object;
pub mod universe;
pub mod singlecharacter;
pub mod number;
pub mod text;
use std;

pub type Universe = universe::Universe;
pub type SingleCharacter = singlecharacter::SingleCharacter;
pub type BoxedObj = boxed_obj::BoxedObj;

// use std::fmt::{Debug, Formatter, Error};

// impl Debug for Object{
//    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
//       write!(f, "Object{{ }}");
//       Ok(())
//    }
// }
