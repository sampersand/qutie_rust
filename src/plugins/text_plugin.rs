use env::Environment;
use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;


use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::text::{Text, Quote, ESCAPE_CHAR};


#[derive(Debug)]
pub struct TextPlugin;

pub static INSTANCE: &'static TextPlugin = &TextPlugin{};
fn escape_char(inp: char) -> char {
   match inp {
      'n' => '\n',
      't' => '\t',
      'r' => '\r',
      '0' => '\0',
      e @ _ => panic!("Unknown escape char: {:?}", inp)
   }
}

impl Plugin for TextPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {

      let start_quote = 
         match env.stream.peek() {
            Some(ref mut c) => 
               if let Some(quote) = Quote::from_char(c.chr) {
                  let __tmp_c = c.take();
                  assert_debug!(eq; __tmp_c, char::from(quote));
                  quote
               } else {
                  return PluginResponse::NoResponse
               },
            _ => return PluginResponse::NoResponse
         };

      let mut text_acc: String = String::new();

      loop {
         let chr = 
            if let Some(ref mut c) = env.stream.peek() {
               c.take()
            } else {
               panic!("Reached eof whilst reading text: {:?}", text_acc)
            };

         if let Some(end_quote) = Quote::from_char(chr) {
            if end_quote == start_quote {
               let quotes = Some((start_quote, end_quote));
               let text = Text::new(text_acc, quotes).to_rc();
               return resp_ok!(text);
            }
         }

         text_acc.push(
            if chr == ESCAPE_CHAR {
               let escaped_char = env.stream.next().expect("Reached EOF whilst parsing escape sequence");
               escape_char(escaped_char)
            } else {
               chr
            });
      } /* end loop */
      unreachable!()
   }
}











