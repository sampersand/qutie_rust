use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub struct DefaultPlugin{}

pub static INSTANCE: DefaultPlugin = DefaultPlugin{};

impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      NextObjectResult::Response( env.stream.next(env) )
   }

   fn handle(&self, token: BoxedObj, env: &mut Environment) -> () {
      env.stream.push(token);
   }
}