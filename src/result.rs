use objects::boxed_obj::BoxedObj;
use objects::boolean::Boolean;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   PlaceHolderForOtherErrors
}

pub type ObjResult = Result<BoxedObj, ObjError>;
pub type OwnedObjResult = Result<BoxedObj, ObjError>;
pub type BoolResult = Result<Box<Boolean>, ObjError>;