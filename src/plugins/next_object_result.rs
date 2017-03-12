use objects::boxed_obj::BoxedObj;
use environment::Environment;
use std;

#[derive(Debug)]
pub enum NextObjectResult {
   NoResponse,
   Retry,
   Response(BoxedObj)
}
