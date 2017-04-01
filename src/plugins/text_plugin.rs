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

      let start_quote = match env.stream.peek() {
                           Some(ref mut c) => if let Some(quote) = Quote::from_char(c.chr) {
                                                 c.take();
                                                 quote
                                              } else {
                                                 return NoResponse
                                              },
                           _ => return NoResponse
                        };

      let mut text_acc: String = String::new();

      loop {
         let chr = match env.stream.peek() {
                      None => panic!("Reached eof whilst reading text: {:?}", text_acc),
                      Some(ref mut c) => c.take(),
                   };
   
         if let Some(end_quote) = Quote::from_char(chr) {
            if end_quote == start_quote {
               let text = Text::new(text_acc, Some((start_quote, end_quote)));
               return Response(ok_rc!(text));
            }
         }
         text_acc.push(if ESCAPE_CHAR == chr {
                          escape_char(env.stream.next().expect("Reached EOF whilst parsing escape sequence"))
                       } else {
                          chr
                       })
      } /* end loop */
      unreachable!()
   }
}











