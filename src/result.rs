use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub enum ObjErr {
   EndOfFile,
   PlaceHolderForOtherErrors
}

pub type ObjResult = Result<BoxedObj, ObjErr>;