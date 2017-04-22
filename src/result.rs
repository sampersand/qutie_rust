use objects::obj_rc::ObjRc;
use objects::boolean::Boolean;
use std::rc::Rc;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   NoSuchKey(ObjRc),
}
pub type ObjResult = Result<ObjRc, ObjError>;
pub type BoolResult = Result<Rc<Boolean>, ObjError>;