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
      match env.stream.next() {
         None => NextObjectResult::NoResponse,
         Some(obj) => NextObjectResult::Response(obj)
      }
   }
}













