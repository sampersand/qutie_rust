use env::Environment;
use objects::obj_rc::ObjRc;

use plugins::plugin::{Plugin, PluginResponse};
use plugins::plugin::PluginResponse::{Retry, NoResponse};
use objects::object::OldObjType;
use objects::user_class::UserClass;
use objects::universe::Universe;
use std::rc::Rc;
use parser::TokenPair;
use result::{ObjError, ObjResult};
use plugins::{symbol_plugin, auto_deref, auto_function_call};

#[derive(Debug)]
pub struct UserFuncPlugin;

pub static INSTANCE: &'static UserFuncPlugin = &UserFuncPlugin{};

fn next_uni(env: &mut Environment) -> Option<ObjRc> {
   let TokenPair(next_obj, _) = env.parser.clone().next_object(env);
   match next_obj {
      Ok(obj) => {
         let obj_clone = obj.clone();
         if let OldObjType::Universe(uni) = obj.obj_type() {
            return Some(obj_clone)
         } else {
            None
         }
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
      if cast_as!(sym, Symbol).sym_val.as_str() != "class" {
         env.stream.feed_back(sym);
         return PluginResponse::NoResponse
      }

      let parents = 
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
            env.stream.feed_back(parents);
            env.stream.feed_back(sym);
            return PluginResponse::NoResponse
         };
      cast_as!(parents, Universe);
      cast_as!(body, Universe);


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
      let parents = parents.qt_exec(env).expect("Couldn't parse function arguments");
      if let Some(pos) = old_func_call_pos {
         env.parser.insert_plugin(pos, auto_function_call::INSTANCE);
      }
      if let Some(pos) = old_deref_pos {
         env.parser.insert_plugin(pos, auto_deref::INSTANCE);
      }


      // use objects::object::Object;
      // use std;
      // let parents: Rc<Universe> = unsafe {
      //    #[allow(mutable_transmutes)] 
      //    let mut a = std::mem::transmute::<&Rc<Object>, &mut Rc<Universe>>(&parents);
      //    let ret = a.clone();
      //    std::ptr::drop_in_place(a as *mut Rc<Universe>);
      //    assert_eq!(Rc::strong_count(&ret), 1);
      //    ret
      // };
      // let body: Rc<Universe> = unsafe {
      //    #[allow(mutable_transmutes)] 
      //    let mut a = std::mem::transmute::<&Rc<Object>, &mut Rc<Universe>>(&body);
      //    let ret = a.clone();
      //    std::ptr::drop_in_place(a as *mut Rc<Universe>);
      //    assert_eq!(Rc::strong_count(&ret), 1);
      //    ret
      // };
      // assert_eq!(Rc::strong_count(&body), 1);
      // assert_eq!(Rc::strong_count(&parents), 1);
      PluginResponse::Response(ok_rc!(UserClass::new(parents, body)))
   }
}














