use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use result::ObjError::EndOfFile;

use env::Environment;
#[derive(Debug)]
pub struct DefaultPlugin;

pub static INSTANCE: &'static DefaultPlugin = &DefaultPlugin{};

impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      if let Some(obj) = env.stream.next_single_char() {
         resp_ok!(obj)
      } else {
         PluginResponse::NoResponse
      }
   }
}













