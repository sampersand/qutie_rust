use plugins::plugin::Plugin;
use environment::Environment;
use plugins::PluginResponse;
use plugins::PluginResponse::{NoResponse, Response};
use objects::number::{Number, NumberType};

#[derive(Debug)]
pub struct NumberPlugin{}

pub static INSTANCE: NumberPlugin = NumberPlugin{};

impl NumberPlugin {
   fn next_base(&self, env: &mut Environment) -> PluginResponse{
      NoResponse
   }
   fn next_float(&self, env: &mut Environment) -> PluginResponse {
      NoResponse
   }
   fn next_int(&self, env: &mut Environment) -> PluginResponse {
      let mut number_acc: String = String::new();
      loop {
         if let Some(obj) = env.stream.peek_char() {
            if obj.source_val.is_digit(10){ number_acc.push(obj.source_val); }
            else { break }
         } else { break }
         env.stream.next(); // this will only occur if a break isnt called
      }
      if number_acc.is_empty() {
         NoResponse
      } else {
         Response(Box::new(Number::new(number_acc.parse::<NumberType>().unwrap()))) // fix this
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














