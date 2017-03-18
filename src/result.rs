use objects::object::Object;
use objects::boolean::Boolean;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   PlaceHolderForOtherErrors
}
pub type ObjResult  = Result<ObjBox, ObjError>;
pub type BoolResult = Result<Box<Boolean>, ObjError>;