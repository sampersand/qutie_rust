use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Retry};
use objects::universe::Universe;
use objects::boxed_obj::BoxedObj;
use objects::single_character::SingleCharacter;
use objects::object::ObjectType;

#[derive(Debug)]
pub struct WhitespacePlugin{}

pub static INSTANCE: WhitespacePlugin = WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      if match env.stream.peek_char() {
         Some(obj) => obj.source_val.is_whitespace(),
         None => false
      } {
         env.stream.next();
         Retry
      } else {
         NoResponse
      }
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {}
}














