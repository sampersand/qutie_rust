use env::Environment;

use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};
use objects::number::Number;
use objects::number;
use result::ObjError::EndOfFile;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: &'static NumberPlugin = &NumberPlugin{};

impl NumberPlugin {

   fn next_base(env: &mut Environment) -> PluginResponse {
      NoResponse
   }

   fn next_float(env: &mut Environment) -> PluginResponse {
      NoResponse
   }

   fn next_int(env: &mut Environment) -> PluginResponse {
      guard!(let Some(c) if c.is_digit(10) = env.stream.peek_char()
             else { return NoResponse });

      let mut number_acc: String = String::new();

      while let Some(c) = env.stream.peek_char() {
         if !c.is_digit(10) { break }
         number_acc.push(c);
         assert_next_eq!(c, env)
      }

      assert!(number_acc.len() > 0);
      let num = Number::new(number_acc.parse::<number::NumberType>().unwrap());
      Response(ok_rc!(num))
   }
}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match NumberPlugin::next_base(env) {
         NoResponse => match NumberPlugin::next_float(env) {
            NoResponse => NumberPlugin::next_int(env),
            o @ _ => o,
         },
         o @ _ => o,
      }
   }
}














