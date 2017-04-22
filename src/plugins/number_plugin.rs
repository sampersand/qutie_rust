use env::Environment;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::number::Number;
use objects::number;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: &'static NumberPlugin = &NumberPlugin{};

impl NumberPlugin {

   fn next_base(_: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn next_float(_: &mut Environment) -> PluginResponse {
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

      assert_debug!(!number_acc.is_empty());
      let num = number_acc.parse::<number::NumberType>();
      assert_debug!(is_ok; num);
      resp_ok!(Number::new(num.expect("Error with converting number string")).to_rc())
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














