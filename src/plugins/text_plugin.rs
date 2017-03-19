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

pub static INSTANCE: TextPlugin = TextPlugin{};

impl Plugin for TextPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let start_quote = match Quote::from_single_char(match_peek_char!(env)) {
         Some(obj) => obj,
         None => return PluginResponse::NoResponse,
      };
      // let start_quote = match env.stream.peek_char(){
      //    Ok(peeked_struct) => {
      //       let peeked_char = peeked_struct.char_val;
      //       if let Some(start_quote) = Quote::from_single_char(peeked_char) {
      //          start_quote
      //       } else {
      //          return PluginResponse::NoResponse
      //       }
      //    }, 
      //    Err(ObjError::EndOfFile) => return PluginResponse::NoResponse,
      //    Err(_) => panic!("Howto deal with non-eof errors"),
      // };

      env.stream.next();
      let mut text_acc: String = String::new();
      let mut result = PluginResponse::NoResponse;
      loop {
         let mut was_escaped = false;

         match env.stream.peek_char() {
            Ok(peeked_struct) => {
               let peeked_char = peeked_struct.char_val;
               if let Some(end_quote) = Quote::from_single_char(peeked_char) {
                  let text = Text::new(text_acc, start_quote, end_quote);
                  result = PluginResponse::Response(Ok(Rc::new(text)));
                  break
               } else {
                  text_acc.push(peeked_char);
                  was_escaped = peeked_char == ESCAPE_CHAR;
               }
            }
            Err(ObjError::EndOfFile) => panic!("Reached EOF whilst parsing string: {:?}", start_quote),
            Err(_) => panic!("Howto deal with non-eof errors")
         }

         let _next_char = env.stream.next();

         if was_escaped {
            text_acc.push(env.stream.peek_char().unwrap().char_val);
            env.stream.next();
         }

      }
      match result {
         PluginResponse::Response(_) => {
            env.stream.next();
            result
         }
         _ => result
      }
   }
}














