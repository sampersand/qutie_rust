use std::rc::Rc;
use env::Environment;
use objects::obj_rc::ObjRc;

use objects::object::ObjType;
use result::ObjError;
use objects::operator::Operator;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{auto_deref, symbol_plugin, universe_plugin};
use parser::TokenPair;
use objects::universe::AccessType;
use objects::symbol::Symbol;

#[derive(Debug)]
pub struct AutoFunctionCall;

pub static INSTANCE: &'static AutoFunctionCall = &AutoFunctionCall{};

impl Plugin for AutoFunctionCall {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.universe.stack.last() {
         Some(obj) =>
            // if !(/*obj.is_a(ObjType::Universe) || */
            if !(obj.is_a(ObjType::Universe) || 
                  obj.is_a(ObjType::BuiltinFunction) || /*obj.is_a(ObjType::BuiltinMethod) ||*/
                  obj.is_a(ObjType::UserFunction) || obj.is_a(ObjType::UserClass)) {
               return PluginResponse::NoResponse
            },
         _ => return PluginResponse::NoResponse
      }

      let args =
         match universe_plugin::INSTANCE.next_object(env) {
           PluginResponse::Response(obj) => obj.unwrap(),
           PluginResponse::NoResponse => return PluginResponse::NoResponse,
           PluginResponse::Retry => panic!("Why is retry being returned from universe?")
         };

      let func = env.universe.stack.pop().unwrap();

      use objects::operator::{call_fn, exec_fn, deref_fn};
      let args = exec_fn(Some(args), None, env).unwrap();
      let response = call_fn(Some(func), Some(args), env);
      PluginResponse::Response(response)
      
   }
}














