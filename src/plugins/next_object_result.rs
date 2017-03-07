use objects::{Object, BoxedObj};
use environment::Environment;
use std;

pub enum NextObjectResult {
   NoResponse,
   Retry,
   Response(BoxedObj)
}
