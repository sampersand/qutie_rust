use objects::obj_rc::ObjRc;
use std::rc::Rc;
use objects::object::Object;
use objects::boolean::Boolean;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   PlaceHolderForOtherErrors
}
pub type ObjResult  = Result<ObjRc, ObjError>;
pub type BoolResult = Result<Rc<Boolean>, ObjError>;