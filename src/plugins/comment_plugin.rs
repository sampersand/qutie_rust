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

      if LINE_START == looked!(env, '_') {  /* `_` can't be LINE_START */
         let first_single_char = env.stream.next().unwrap();
         match env.stream.clone().looked() {
            Some(c) if *c == LINE_START => {
               while let Some(ref mut c) = env.stream.peek() {
                  if c.chr == LINE_ENDL { break }
                  c.take();
                  // assert_next_eq!(c, env)
               }
               return Retry
            },
            _ => env.stream.feed(first_single_char),
         }
      }
      NoResponse
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














