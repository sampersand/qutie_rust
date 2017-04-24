use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

use env::Environment;
#[derive(Debug)]
pub struct DefaultPlugin;

pub static INSTANCE: &'static DefaultPlugin = &DefaultPlugin{};

impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      if let Some(obj) = env.stream.next_single_char() {
         assert_debug!(false, "Shouldn't be defaulting");
         resp_ok!(obj)
      } else {
         PluginResponse::NoResponse
      }
   }
}













