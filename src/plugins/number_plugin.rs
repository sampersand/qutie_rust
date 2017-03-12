use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use plugins::next_object_result::NextObjectResult::{NoResponse, Response};
use objects::number::Number;

#[derive(Debug)]
pub struct NumberPlugin{}

pub static INSTANCE: NumberPlugin = NumberPlugin{};

impl NumberPlugin {
   fn next_base(&self, env: &mut Environment) -> NextObjectResult{
      NoResponse
   }
   fn next_float(&self, env: &mut Environment) -> NextObjectResult {
      NoResponse
   }
   fn next_int(&self, env: &mut Environment) -> NextObjectResult {
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
         type NumberReturnType = i32;
         Response(
            Box::new(
               Number::<NumberReturnType>::new(
                  number_acc.parse::<NumberReturnType>().unwrap()
               )
            )
         )
      }
   }

}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      match self.next_base(env) {
         NoResponse => match self.next_float(env) {
            NoResponse => self.next_int(env),
            e @ _ => e,
         },
         e @ _ => e,
      }
   }
}














