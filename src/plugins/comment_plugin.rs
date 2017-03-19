use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

#[derive(Debug)]
pub struct CommentPlugin;

pub static INSTANCE: CommentPlugin = CommentPlugin{};

const SINGLE_LINE_START: char = '#';
const SINGLE_LINE_ENDL: char = '\n';

impl CommentPlugin {
   fn multi_line(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn single_line(env: &mut Environment) -> PluginResponse{
      let is_comment = match env.stream.peek_char() {
         Ok(peeked_struct) => match peeked_struct.char_val {
            SINGLE_LINE_START => true,
            _ => false,
         },
         Err(ObjError::EndOfFile) => false,
         Err(err) => panic!("Don't know how to handle error: {:?}", err),
      };
      if is_comment {
         env.stream.next(); // to get rid of the whitespace
         loop {
            match env.stream.peek_char() {
               Ok(peeked_struct) if peeked_struct.char_val == SINGLE_LINE_ENDL => break,
               Err(ObjError::EndOfFile) => break,
               Ok(obj) => {},
               Err(err) => panic!("Don't know how to handle error: {:?}", err),
            }
            env.stream.next();
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














