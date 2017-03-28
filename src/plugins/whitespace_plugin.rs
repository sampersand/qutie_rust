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
      match env.stream.peek() {
         Some(c) if c.is_whitespace() => { assert_next_eq!(c, env); Retry },
         _ => NoResponse
      }
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling whitespace
   } 
}














