use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;

#[derive(Debug)]
pub struct CommentPlugin;

pub static INSTANCE: &'static CommentPlugin = &CommentPlugin{};

impl Plugin for CommentPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      const LINE_START: char = '/';
      const LINE_ENDL: char = '\n';
      
      let first = // first slash
         match env.stream.peek() {
            Some(ref mut c) if c.chr == LINE_START => c.take(),
            _ => return PluginResponse::NoResponse
         };
      let has_second = // has second slash
         match env.stream.peek() {
            Some(ref mut c) if c.chr == LINE_START => {c.take(); true},
            _ => false
         };
      if has_second { //if it's `//` and not somethign else (e.g. `/2` as in `1/2`)
         while let Some(ref mut c) = env.stream.peek() {
            if c.take() == LINE_ENDL { break }
         }
         PluginResponse::Retry
      } else {
         env.stream.feed(first);
         PluginResponse::NoResponse
      }
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling comments
   }
}














