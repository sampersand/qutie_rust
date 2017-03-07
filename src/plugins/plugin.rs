use objects::{Object, BoxedObj};
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use std;
pub trait Plugin : std::fmt::Debug {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult;
   fn handle(&self, token: BoxedObj, env: &mut Environment) -> ();
}