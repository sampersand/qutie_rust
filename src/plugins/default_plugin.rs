use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{Response, NoResponse};
use result::ObjError::EndOfFile;

use env::Environment;
#[derive(Debug)]
pub struct DefaultPlugin;

pub static INSTANCE: &'static DefaultPlugin = &DefaultPlugin{};

impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      if let Some(obj) = env.stream.next_single_char() {
         Response(Ok(obj))
      } else {
         NoResponse
      }
   }
}













