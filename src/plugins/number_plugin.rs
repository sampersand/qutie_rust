use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::number::{Number, NumberType};
use result::ObjError;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: NumberPlugin = NumberPlugin{};

impl NumberPlugin {
   fn next_base(&self, env: &mut Environment) -> PluginResponse{
      PluginResponse::NoResponse
   }
   fn next_float(&self, env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }
   fn next_int(&self, env: &mut Environment) -> PluginResponse {
      let mut number_acc: String = String::new();

      loop {
         match env.stream.peek_char() {
            Ok(peeked_single_character) => {
               let peeked_char = peeked_single_character.source_val;
               if peeked_char.is_digit(10){
                  number_acc.push(peeked_char);
               } else {
                  break
               }
            }, 
            Err(ObjError::EndOfFile) => break,
            Err(_) => panic!("IDK How to deal with non-eof errors")
         }
         let _next_char = env.stream.next(); // and ignore it
      }

      if number_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         let raw_num = number_acc.parse::<NumberType>().unwrap();
         let num_struct = Number::new(raw_num);
         PluginResponse::Response(Ok(Box::new(num_struct)))
      }
   }

}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match self.next_base(env) {
         PluginResponse::NoResponse => match self.next_float(env) {
            PluginResponse::NoResponse => self.next_int(env),
            e @ _ => e,
         },
         e @ _ => e,
      }
   }
}














