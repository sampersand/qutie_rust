use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: WhitespacePlugin = WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let is_whitespace = match env.stream.peek_char() {
         Ok(peeked_struct) => peeked_struct.char_val.is_whitespace(),
         Err(_) => false
      };
      if is_whitespace {
         env.stream.next(); // to get rid of the whitespace
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling whitespace
   } 
}














