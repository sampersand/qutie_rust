use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub enum ObjErr {
   EndOfFile,
}

pub type ObjResult = Result<BoxedObj, ObjErr>;