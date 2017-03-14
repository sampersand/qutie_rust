use objects::universe::Universe;
use objects::object::Object;
use objects::boxed_obj::BoxedObj;
use environment::Environment;
use result::ObjResult;
use std::fmt::Debug;

#[derive(Debug)]
pub enum PluginResponse {
   NoResponse,
   Retry,
   Response(ObjResult)
}

pub trait Plugin : Debug {
   fn next_object(&self, env: &mut Environment) -> PluginResponse;
   fn handle(&self, token: BoxedObj, env: &mut Environment)->(){ env.universe.push(token); }
}