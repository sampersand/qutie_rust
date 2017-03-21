use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

#[derive(Debug)]
pub struct CommentPlugin;

pub static INSTANCE: &'static CommentPlugin = &CommentPlugin{};

impl CommentPlugin {
   fn multi_line(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn single_line(env: &mut Environment) -> PluginResponse{
      const LINE_START: char = '#';
      const LINE_ENDL: char = '\n';
      if LINE_START == peek_char!(env, EndOfFile => '_') {  /* `_` can't be LINE_START */
         loop {
            env.stream.next();
            if LINE_ENDL == peek_char!(env, EndOfFile => break) {
               break
            }
         }
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
}
impl Plugin for CommentPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match CommentPlugin::multi_line(env) {
         PluginResponse::NoResponse => CommentPlugin::single_line(env),
         other @ _ => other,
      }
   }
   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling comments
   }
}














