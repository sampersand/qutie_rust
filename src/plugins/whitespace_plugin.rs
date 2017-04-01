use env::Environment;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{Retry, NoResponse};

use objects::obj_rc::ObjRc;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: &'static WhitespacePlugin = &WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.stream.peek() {
         Some(ref mut c) if c.is_whitespace() => {
            c.take();
            Retry
         },
         _ => NoResponse
      }
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling whitespace
   } 
}














