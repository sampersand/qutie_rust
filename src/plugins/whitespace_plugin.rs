use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::boxed_obj::BoxedObj;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: WhitespacePlugin = WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let is_whitespace = match env.stream.peek_char() {
         Some(obj) => obj.source_val.is_whitespace(),
         None => false
      };
      if is_whitespace {
         env.stream.next();
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {}
}














