use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;

#[derive(Debug)]
pub struct WhitespacePlugin;

pub static INSTANCE: WhitespacePlugin = WhitespacePlugin{};

impl Plugin for WhitespacePlugin {
   fn next_object(&self,
                  stream: &mut Universe, // stream
                  _: &mut Universe, // enviro
                  _: &Parser,       // parser
                 ) -> PluginResponse {
      let is_whitespace = match stream.peek_char() {
         Ok(peeked_struct) => peeked_struct.source_val.is_whitespace(),
         Err(_) => false
      };
      if is_whitespace {
         stream.next(); // to get rid of the whitespace
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
   fn handle(&self,
             _: ObjBox, // token
             _: &mut Universe, // stream
             _: &mut Universe, // enviro
             _: &Parser,       // parser
            ) {} // we shouldn't be handling whitespace
}














