use objects::obj_rc::ObjRc;
use parser::Parser;

use result::{ObjResult, ObjError};
use std::fmt::Debug;

use env::Environment;

#[derive(Debug)]
pub enum PluginResponse {
   NoResponse,
   Retry,
   Response(ObjResult)
}

pub trait Plugin : Debug {
   fn next_object(&self, env: &mut Environment) -> PluginResponse;
   fn handle(&self, token: ObjRc, env: &mut Environment) {
      env.universe.push(token);
   }
}