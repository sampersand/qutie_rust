use objects::universe::Universe;
use objects::object::Object;
use objects::boxed_obj::BoxedObj;
use environment::Environment;
use plugins::PluginResponse;
use std;
pub trait Plugin : std::fmt::Debug {
   fn next_object(&self, env: &mut Environment) -> PluginResponse;
   fn handle(&self, token: BoxedObj, env: &mut Environment)->(){ env.universe.push(token); }
}