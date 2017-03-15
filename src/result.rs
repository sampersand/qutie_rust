use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub enum ObjErr {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   PlaceHolderForOtherErrors
}

pub type ObjResult = Result<BoxedObj, ObjErr>;