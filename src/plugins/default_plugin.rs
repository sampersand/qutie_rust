use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::boxed_obj::BoxedObj;

use result::ObjErr;
#[derive(Debug)]
pub struct DefaultPlugin;

pub static INSTANCE: DefaultPlugin = DefaultPlugin{};

use objects::universe::Universe;
impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.stream.next() {
         Ok(obj) => PluginResponse::Response(Ok(obj)),
         Err(ObjErr::EndOfFile) => PluginResponse::NoResponse,
         Err(err) => panic!("Don't know how to deal with err: {:?}!", err)
      }
   }
}













