use env::Environment;
use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;


use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::text::{Text, Quote, ESCAPE_CHAR};
use result::ObjError;

#[derive(Debug)]
pub struct TextPlugin;

pub static INSTANCE: &'static TextPlugin = &TextPlugin{};

impl Plugin for TextPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {

      let start_quote = match Quote::from_char(peek_char!(env)) {
         Some(obj) => obj,
         None => return PluginResponse::NoResponse,
      };

      env.stream.next();

      let mut text_acc: String = String::new();

      loop {
         let mut was_escaped = false;
         let peeked_char = peek_char!(env, EndOfFile => panic!("Reached EOF whilst reading text: {:?}", text_acc));
   
         if let Some(end_quote) = Quote::from_char(peeked_char) {
            env.stream.next();
            return ok_rc!(RESP; Text::new(text_acc, [start_quote, end_quote]));
         }
         let char_to_push = if ESCAPE_CHAR == peeked_char {
                               env.stream.next();
                               text_acc.push(peek_char!(env, EndOfFile => panic!("Escape during string at EOF")));
                            } else {
                               text_acc.push(peeked_char);
                            };
         env.stream.next();
      }
      
      panic!("Why would anything get here")
   }
}














