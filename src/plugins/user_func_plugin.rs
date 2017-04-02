use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::{Plugin, PluginResponse};
use plugins::plugin::PluginResponse::{Retry, NoResponse};
use objects::object::{ObjType, ObjWrapper};
use objects::user_function::UserFunction;
use objects::universe::Universe;
use objects::symbol::Symbol;
use objects::object::Object;
use std::rc::Rc;
use parser::TokenPair;
use result::{ObjError, ObjResult};
use plugins::{symbol_plugin, auto_deref, auto_function_call};

#[derive(Debug)]
pub struct UserFuncPlugin;

pub static INSTANCE: &'static UserFuncPlugin = &UserFuncPlugin{};

fn next_uni(env: &mut Environment) -> Option<Rc<Universe>> {
   let TokenPair(next_obj, _) = env.parser.clone().next_object(env);
   match next_obj {
      Ok(obj) => 
         if obj.is_a(ObjType::Universe) {
            Some(cast_as!(obj, Universe))
         } else {
            None
         },
      Err(ObjError::EndOfFile) => None,
      Err(err) => panic!("unknown error: {:?}", err)
   }
}
impl Plugin for UserFuncPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let sym = match symbol_plugin::INSTANCE.next_object(env) {
         PluginResponse::Retry => panic!("Why is retry being returned from the symbol plugin?"),
         PluginResponse::NoResponse => return PluginResponse::NoResponse,
         PluginResponse::Response(res) => match res {
            Ok(sym) => sym,
            Err(err) => panic!("What to do with the error: {:?}", err)
         }
      };
      if cast_as!(CL; sym, Symbol).sym_val.as_str() != "func" {
         env.stream.feed_back(sym);
         return PluginResponse::NoResponse
      }

      let args = 
         if let Some(uni) = next_uni(env) {
            uni
         } else {
            env.stream.feed_back(sym);
            return PluginResponse::NoResponse
         };
      let body = 
         if let Some(uni) = next_uni(env) {
            uni
         } else {
            env.stream.feed_back(args);
            env.stream.feed_back(sym);
            return PluginResponse::NoResponse
         };


      let old_deref_pos =
         if env.parser.has_plugin(auto_deref::INSTANCE) {
            Some(env.parser.del_plugin(auto_deref::INSTANCE))
         } else {
            None
         };
      let old_func_call_pos =
         if env.parser.has_plugin(auto_function_call::INSTANCE) {
            Some(env.parser.del_plugin(auto_function_call::INSTANCE))
         } else {
            None
         };
      
      let args = cast_as!(args.qt_exec(env).expect("Couldn't parse function arguments"), Universe);

      if let Some(pos) = old_func_call_pos {
         env.parser.insert_plugin(pos, auto_function_call::INSTANCE);
      }
      if let Some(pos) = old_deref_pos {
         env.parser.insert_plugin(pos, auto_deref::INSTANCE);
      }

      PluginResponse::Response(ok_rc!(UserFunction::new(args, body)))
   }
}














