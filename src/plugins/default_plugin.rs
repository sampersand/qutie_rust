use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub struct DefaultPlugin{}

pub static INSTANCE: DefaultPlugin = DefaultPlugin{};

use objects::universe::Universe;
impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.stream.next() {
         None => PluginResponse::NoResponse,
         Some(obj) => PluginResponse::Response(Ok(obj))
      }
   }
}













