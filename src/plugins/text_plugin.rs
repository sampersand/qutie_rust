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
fn escape_char(inp: char) -> char {
   match inp {
      'n' => '\n',
      't' => '\t',
      'r' => '\r',
      '0' => '\0',
      e @ _ => e,
   }
}

impl Plugin for TextPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {

      let start_quote = match Quote::from_char(peek_char!(env)) {
                           Some(obj) => obj,
                           None => return PluginResponse::NoResponse,
                        };

      assert_next_eq!(start_quote.to_char(), env);

      let mut text_acc: String = String::new();

      loop {
         let peeked_char = peek_char!(env, EndOfFile => panic!("Reached EOF whilst reading text: {:?}", text_acc));
   
         if let Some(end_quote) = Quote::from_char(peeked_char) {
            if end_quote == start_quote {
               assert_next_eq!(start_quote.to_char(), env);
               return ok_rc!(RESP; Text::new(text_acc, [start_quote, end_quote]));
            }
         }
         if ESCAPE_CHAR == peeked_char {
            assert_eq!(ESCAPE_CHAR, peeked_char);
            assert_next_eq!(ESCAPE_CHAR, env);

            let next_char = peek_char!(env, EndOfFile => panic!("Escape during string at EOF"));
            text_acc.push(escape_char(next_char));
            assert_next_eq!(next_char, env);
         } else {
            text_acc.push(peeked_char);
            assert_next_eq!(peeked_char, env);
         }
      } /* end loop */
      unreachable!()    
   }
}














