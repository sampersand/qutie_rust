use std::rc::Rc;
use env::Environment;
use objects::obj_rc::ObjRc;

use objects::object::ObjType;
use result::ObjError;
use objects::operator::Operator;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{symbol_plugin, operator_plugin};
use parser::TokenPair;

#[derive(Debug)]
pub struct AutoDeref;

pub static INSTANCE: &'static AutoDeref = &AutoDeref{};

impl Plugin for AutoDeref {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let sym = match symbol_plugin::INSTANCE.next_object(env) {
         PluginResponse::Retry => panic!("Why is retry being returned from the symbol plugin?"),
         PluginResponse::NoResponse => return PluginResponse::NoResponse,
         PluginResponse::Response(res) => match res {
            Err(err) => panic!("What to do with the error: {:?}", err),
            Ok(sym) => sym
         }
      };
      /* this will work weirdly with whitespace and custom operators */ 
      let TokenPair(next_obj, _) = env.parser.clone().next_object(env);
      let no_response = match next_obj {
         Ok(obj) => {
            env.stream.feed_back(obj.clone());
            if let ObjType::Operator(oper) = obj.obj_type() {
                oper.sigil.as_str() == "=" // Fails w/ custom operators
                // || match env.universe.stack.last(){
                //   None => false,
                //   Some(last) => if let ObjType::Operator(last_oper) = last.obj_type() {
                //      last_oper.sigil.as_str() == "."
                //   } else { false }
                // }

            } else {
                false
            }
         }, 
         Err(ObjError::EndOfFile) => false,
         Err(err) => panic!("unknown error: {:?}", err)
      };
      if no_response {
         env.stream.feed_back(sym);
         PluginResponse::NoResponse
      } else {
         use objects::operator::deref_fn;
         PluginResponse::Response(deref_fn(Some(sym), None, env))
      }
   }
}














