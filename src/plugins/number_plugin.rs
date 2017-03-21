use env::Environment;

use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::number::{Number, NumberType};
use result::ObjError;

#[derive(Debug)]
pub struct NumberPlugin;

pub static INSTANCE: &'static NumberPlugin = &NumberPlugin{};

impl NumberPlugin {
   fn next_base(env: &mut Environment) -> PluginResponse{
      PluginResponse::NoResponse
   }
   fn next_float(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }
   fn next_int(env: &mut Environment) -> PluginResponse {
      let mut number_acc: String = String::new();

      loop {
         match peek_char!(env, EndOfFile => break) {
            c if c.is_digit(10) => number_acc.push(c),
            _ => break
         }
         env.stream.next(); // and ignore it
      }

      if number_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         ok_rc!(RESP; Number::new(number_acc.parse::<NumberType>().unwrap()))
      }
   }

}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match NumberPlugin::next_base(env) {
         PluginResponse::NoResponse => match NumberPlugin::next_float(env) {
            PluginResponse::NoResponse => NumberPlugin::next_int(env),
            other @ _ => other,
         },
         other @ _ => other,
      }
   }
}














