use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{Retry, NoResponse};
use objects::object::ObjType;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: &'static WhitespacePlugin = &WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let peeked_char = peek_char!(env, EndOfFile => 'a');
      if peeked_char.is_whitespace() {
         assert_next_eq!(peeked_char, env);
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling whitespace
   } 
}














