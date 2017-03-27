use objects::obj_rc::ObjRc;
use std::rc::Rc;
use objects::object::Object;
use objects::boolean::Boolean;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   NoSuchKey(ObjRc),
   PlaceHolderForOtherErrors
}
pub type ObjResult = Result<ObjRc, ObjError>;
