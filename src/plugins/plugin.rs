use objects::universe::Universe;
use objects::object::Object;
use objects::boxed_obj::BoxedObj;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use std;
pub trait Plugin : std::fmt::Debug {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult;
   fn handle(&self, token: BoxedObj, env: &mut Environment) -> () {
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), env.parser);
      env.universe.push(token, to_pass);
   }
}