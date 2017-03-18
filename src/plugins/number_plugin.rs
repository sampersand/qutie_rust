use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::number::{Number, NumberType};
use result::ObjError;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: NumberPlugin = NumberPlugin{};

impl NumberPlugin {
   fn next_base(&self,
                stream: &mut Universe, // stream
                enviro: &mut Universe, // enviro
                parser: &Parser,       // parser
               ) -> PluginResponse{
      PluginResponse::NoResponse
   }
   fn next_float(&self,
                 stream: &mut Universe, // stream
                 enviro: &mut Universe, // enviro
                 parser: &Parser,       // parser
                ) -> PluginResponse {
      PluginResponse::NoResponse
   }
   fn next_int(&self,
               stream: &mut Universe, // stream
               _: &mut Universe, // enviro
               _: &Parser,       // parser
              ) -> PluginResponse {
      let mut number_acc: String = String::new();

      loop {
         match stream.peek_char() {
            Ok(peeked_single_character) => {
               let peeked_char = peeked_single_character.char_val;
               if peeked_char.is_digit(10){
                  number_acc.push(peeked_char);
               } else {
                  break
               }
            }, 
            Err(ObjError::EndOfFile) => break,
            Err(_) => panic!("IDK How to deal with non-eof errors")
         }
         let _next_char = stream.next(); // and ignore it
      }

      if number_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         let raw_num = number_acc.parse::<NumberType>().unwrap();
         let num_struct = Number::new(raw_num);
         PluginResponse::Response(Ok(Rc::new(num_struct)))
      }
   }

}

impl Plugin for NumberPlugin {
   fn next_object(&self,
                  stream: &mut Universe, // stream
                  enviro: &mut Universe, // enviro
                  parser: &Parser,       // parser
                 ) -> PluginResponse {
      match self.next_base(stream, enviro, parser) {
         PluginResponse::NoResponse => match self.next_float(stream, enviro, parser) {
            PluginResponse::NoResponse => self.next_int(stream, enviro, parser),
            e @ _ => e,
         },
         e @ _ => e,
      }
   }
}














