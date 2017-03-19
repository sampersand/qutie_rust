use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use result::ObjError;

use env::Environment;
#[derive(Debug)]
pub struct DefaultPlugin;

pub static INSTANCE: DefaultPlugin = DefaultPlugin{};

impl Plugin for DefaultPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.stream.next() {
         Ok(obj) => PluginResponse::Response(Ok(obj)),
         Err(ObjError::EndOfFile) => PluginResponse::NoResponse,
         Err(err) => panic!("Don't know how to deal with err: {:?}!", err)
      }
   }
}













