use objects::obj_rc::ObjRc;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   NoSuchKey(ObjRc),
   PlaceHolderForOtherErrors
}
pub type ObjResult = Result<ObjRc, ObjError>;
