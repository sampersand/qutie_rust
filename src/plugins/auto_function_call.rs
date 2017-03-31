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
      let args = match universe_plugin::INSTANCE.next_object(env) {
                    PluginResponse::Response(obj) => qt_try!(obj),
                    PluginResponse::NoResponse => return PluginResponse::NoResponse,
                    PluginResponse::Retry => panic!("Why is retry being returned from universe?")
                 };
      let func = env.universe.stack.pop().unwrap();
      use objects::operator::{call_fn, exec_fn, deref_fn};
      let args = qt_try!(exec_fn(Some(args), None, env));
      let response = call_fn(Some(func), Some(args), env);
      PluginResponse::Response(response)

      // let mut was_period = false;
      // if let Some(obj) = env.universe.stack.last() {
      //    match obj.obj_type() {
      //       ObjType::Operator(obj) if obj.sigil == "." => was_period = true,
      //       _ => {}
      //    }
      // }
      // if was_period {
      //    let ret = symbol_plugin::INSTANCE.next_object(env);
      //    env.stream.feed(',');
      //    return ret;
      // }

      // let func = match symbol_plugin::INSTANCE.next_object(env){
      //    PluginResponse::Response(obj) => qt_try!(obj),
      //    PluginResponse::NoResponse => return PluginResponse::NoResponse,
      //    PluginResponse::Retry => panic!("Why is retry being returned from symbol?")
      // };

      // cast_as!(func, Symbol);

      // let args = qt_try!(env.parser.clone().next_object(env).0,
      //                    EndOfFile => {
      //                       env.stream.feed_back(func);
      //                       return PluginResponse::NoResponse;
      //                    });

      // let is_uni = match args.obj_type() {
      //    ObjType::Universe(_) => true,
      //    _ => false
      // };
      // if is_uni {
      //    use objects::operator::{call_fn, exec_fn, deref_fn};
      //    let args = qt_try!(exec_fn(Some(args), None, env));
      //    let func = qt_try!(deref_fn(Some(func), None, env));
      //    let response = call_fn(Some(func), Some(args), env);
      //    PluginResponse::Response(response)
      // } else {
      //    env.stream.feed_back(args);
      //    env.stream.feed_back(func);   
      //    PluginResponse::NoResponse
      // }
      
   }
}














