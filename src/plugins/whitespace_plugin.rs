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
         Ok(peeked_struct) => peeked_struct.source_val.is_whitespace(),
         Err(_) => false
      };
      if is_whitespace {
         env.stream.next(); // to get rid of the whitespace
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {} // we shouldn't be handling whitespace
}














