use std::rc::Rc;
use env::Environment;
use objects::obj_rc::ObjRc;

use objects::object::ObjType;
use objects::symbol::Symbol;
use result::ObjError;
use objects::operator::Operator;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{auto_deref, symbol_plugin};
use parser::TokenPair;

#[derive(Debug)]
pub struct AutoFunctionCall;

pub static INSTANCE: &'static AutoFunctionCall = &AutoFunctionCall{};

impl Plugin for AutoFunctionCall {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      
      // let mut funcs = vec![];
      // loop {
      //    let next_sym = match symbol_plugin::INSTANCE.next_object(env){
      //       PluginResponse::Response(obj) => qt_try!(obj),
      //       PluginResponse::NoResponse => return PluginResponse::NoResponse,
      //       PluginResponse::Retry => panic!("Why is retry being returned from symbol?")
      //    };
      //    func.push(cast_as!(next_sym, Symbol));
      //    let peeked = env.stream.peek().unwrap();
      //    if peeked.chr == '.' {
      //       peeked.take()
      //    } else {
      //       break
      //    }
      // }

      // let func = rc!(Symbol::new(func));

      
      // let is_sym = match func.obj_type() {
      //    ObjType::Symbol(_) => true,
      //    _ => false
      // };
      // if !is_sym {
      //    env.stream.feed_back(func);
      //    return PluginResponse::NoResponse;
      // }
      match env.universe.stack.last() {
         None => return PluginResponse::NoResponse,
         Some(obj) => match obj.obj_type() {
            ObjType::Universe(_) | ObjType::BuiltinFunction(_) => {},
            _ => return PluginResponse::NoResponse
         }  
      }
      println!("{:?}", env.universe.stack);
      println!("{:?}", env.stream);
      let old_pos = env.parser.del_plugin(INSTANCE);
      println!("{:?}", old_pos);
      let args = env.parser.clone().next_object(env).0.unwrap();
      println!("{:?}", args);
      env.parser.insert_plugin(old_pos, INSTANCE);

      let is_uni = match args.obj_type() {
         ObjType::Universe(_) => true,
         _ => false
      };

      if is_uni {
         use objects::operator::{call_fn, exec_fn, deref_fn};
         let args = qt_try!(exec_fn(Some(args), None, env));
         let func = env.universe.stack.last().unwrap().clone();//qt_try!(deref_fn(Some(func), None, env));

         let response = call_fn(Some(func), Some(args), env);
         PluginResponse::Response(response)
      } else {
         env.stream.feed_back(args);
         PluginResponse::NoResponse
      }
      
   }
}














