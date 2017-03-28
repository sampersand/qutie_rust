use env::Environment;
use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;


use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};
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

      let start_quote = if let Some(obj) = Quote::from_char(looked!(env)) {
                           obj
                        } else {
                           return NoResponse
                        };

      assert_next_eq!(start_quote.to_char(), env);

      let mut text_acc: String = String::new();

      loop {
         let lookeded_char = looked!(env, panic!("Reached EOF whilst reading text: {:?}", text_acc));
   
         match Quote::from_char(lookeded_char) {
            Some(end_quote) if end_quote == start_quote => {
               assert_next_eq!(start_quote.to_char(), env);
               let text = Text::new(text_acc, [start_quote, end_quote]);
               return Response(ok_rc!(text));
            },
            _ => text_acc.push(if ESCAPE_CHAR == lookeded_char {
                                   assert_eq!(ESCAPE_CHAR, lookeded_char);
                                   assert_next_eq!(ESCAPE_CHAR, env);

                                   let next_char = looked!(env, panic!("Escape during string at EOF"));
                                   assert_next_eq!(next_char, env);
                                   escape_char(next_char)
                                } else {
                                   assert_next_eq!(lookeded_char, env);
                                   lookeded_char
                                }
                               )
         }
      } /* end loop */
      unreachable!()
   }
}














