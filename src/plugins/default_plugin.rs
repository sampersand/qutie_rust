use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub struct DefaultPlugin{}

pub static INSTANCE: DefaultPlugin = DefaultPlugin{};

use objects::universe::Universe;
impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), env.parser);
      match env.stream.next( to_pass ) {
         None => NextObjectResult::NoResponse,
         Some(e) => NextObjectResult::Response(e)
      }
   }

   fn handle(&self, token: BoxedObj, env: &mut Environment) -> () {
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), env.parser);
      env.universe.push(token, to_pass);
   }
}













