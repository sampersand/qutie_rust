use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError::EndOfFile;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response, Retry};

#[derive(Debug)]
pub struct CommentPlugin;

pub static INSTANCE: &'static CommentPlugin = &CommentPlugin{};

impl CommentPlugin {
   fn multi_line(env: &mut Environment) -> PluginResponse {
      NoResponse
   }

   fn single_line(env: &mut Environment) -> PluginResponse{
      const LINE_START: char = '/';
      const LINE_ENDL: char = '\n';

      let first = match env.stream.peek() {
                     Some(ref mut c) if c.chr == LINE_START => c.take(),
                     _ => return NoResponse
                  };
      let use_scnd = match env.stream.peek() {
                        Some(ref mut c) if c.chr == LINE_START => {c.take(); true},
                        _ => false
                     };
      if use_scnd {
         while let Some(ref mut c) = env.stream.peek() {
            if c.take() == LINE_ENDL {
               break
            }
         }
         Retry
      } else {
         env.stream.feed(first);
         NoResponse
      }
   }
}
impl Plugin for CommentPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match CommentPlugin::multi_line(env) {
         NoResponse => CommentPlugin::single_line(env),
         o @ _ => o,
      }
   }
   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling comments
   }
}














