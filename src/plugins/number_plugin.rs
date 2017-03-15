use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::number::{Number, NumberType};

#[derive(Debug)]
pub struct NumberPlugin{}

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
         if let Ok(peeked_single_character) = env.stream.peek_char(){
            let peeked_char = peeked_single_character.source_val;
            if peeked_char.is_digit(10){
               number_acc.push(peeked_char);
               env.stream.next(); // and ignore it
            } else {
               break
            }
         } else {
            break
         }
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
         NoResponse => match self.next_float(env) {
            NoResponse => self.next_int(env),
            e @ _ => e,
         },
         e @ _ => e,
      }
   }
}














