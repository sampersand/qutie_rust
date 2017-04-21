use std::rc::Rc;
use env::Environment;
use objects::obj_rc::ObjRc;

use objects::object::{ObjType, ObjWrapper};
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
      if let Some(obj) = env.universe.stack.last(){
         if obj.is_a(ObjType::Operator) {
            if cast_as!(CL; obj, Operator).sigil == "." {
               return PluginResponse::NoResponse
            }
         }
      }
      let sym = match symbol_plugin::INSTANCE.next_object(env) {
         PluginResponse::Retry => unreachable!("Why is retry being returned from the symbol plugin?"),
         PluginResponse::NoResponse => return PluginResponse::NoResponse,
         PluginResponse::Response(res) => match res {
            Err(err) => panic!("What to do with the error: {:?}", err),
            Ok(sym) => sym
         }
      };
      // use objects::obj_rc::ObjRcWrapper;
      // if let Some(val) = env.universe.locals.get(&ObjRcWrapper(sym.clone())) {
      //    if val.is_a(ObjType::Operator) {
      //       println!("{:?}", sym);
      //       env.stream.feed_back(sym);
      //       return PluginResponse::NoResponse;
      //    }
      // }

      // this will work weirdly with whitespace and custom operators 
      let TokenPair(next_obj, _) = env.parser.clone().next_object(env);
      let no_response = match next_obj {
         Ok(obj) => {
            env.stream.feed_back(obj.clone());
            use objects::single_character::SingleCharacter;
            env.stream.feed_back(SingleCharacter::new(' ').to_rc());
            obj.is_a(ObjType::Operator) && cast_as!(obj, Operator).sigil == "="
         }, 
         Err(ObjError::EndOfFile) => false,
         Err(err) => unreachable!("unknown error: {:?}", err)
      };
      if no_response {
         env.stream.feed_back(sym);
         PluginResponse::NoResponse
      } else {
         use objects::operator::deref_fn;
         let derefed = deref_fn(Some(sym.clone()), None, env);
         if let Ok(obj) = derefed {
            if obj.is_a(ObjType::Operator) {
               env.stream.feed_back(sym);
               PluginResponse::NoResponse
            } else {
               resp_ok!(obj)
            }
         } else {
            PluginResponse::Response(derefed)
         }
      }
      
   }
}














