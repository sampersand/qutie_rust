use env::Environment;

use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::number::Number;
use objects::number;
use result::ObjError::EndOfFile;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: &'static NumberPlugin = &NumberPlugin{};

impl NumberPlugin {

   fn next_base(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn next_float(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn next_int(env: &mut Environment) -> PluginResponse {
      match env.stream.peek() {
         Some(ref c) if c.is_digit(10) => {},
         _ => return PluginResponse::NoResponse
      }

      let mut number_acc: String = String::new();

      while let Some(ref mut c) = env.stream.peek() {
         if !c.is_digit(10) { break }
         number_acc.push(c.take());
      }

      assert!(0 < number_acc.len());
      let num = Number::new(number_acc.parse::<number::NumberType>().unwrap());
      resp_ok!(rc; num)
   }
}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match NumberPlugin::next_base(env) {
         PluginResponse::NoResponse =>
            match NumberPlugin::next_float(env) {
               PluginResponse::NoResponse => NumberPlugin::next_int(env),
               o @ _ => o,
            },
         o @ _ => o
      }
   }
}














