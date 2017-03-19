use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: WhitespacePlugin = WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      if match_peek_char!(env, EndOfFile => 'a').is_whitespace() {
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














