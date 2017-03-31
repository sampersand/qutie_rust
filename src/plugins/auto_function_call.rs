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
         Some(obj) => match obj.obj_type() {
            ObjType::Universe(_) | ObjType::BuiltinFunction(_) |
            ObjType::UserFunction(_) | ObjType::UserClass(_) => {},
            _ => return PluginResponse::NoResponse
         },
         _ => return PluginResponse::NoResponse
      }

      let args =
         match universe_plugin::INSTANCE.next_object(env) {
           PluginResponse::Response(obj) => qt_try!(obj),
           PluginResponse::NoResponse => return PluginResponse::NoResponse,
           PluginResponse::Retry => panic!("Why is retry being returned from universe?")
         };

      let func = env.universe.stack.pop().unwrap();

      let do_pass_self =
         if let ObjType::UserFunction(func) = func.obj_type() {
            func.is_method()
         } else {
            false
         };

      use objects::operator::{call_fn, exec_fn, deref_fn, __set_fn};
      let args = qt_try!(exec_fn(Some(args), None, env));
      let response = call_fn(Some(func), Some(args), env);
      PluginResponse::Response(response)
      
   }
}














