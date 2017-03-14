use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::boxed_obj::BoxedObj;
use objects::object::{Object, FunctionError};
use objects::null::Null;
use objects::operator::{Operator, OPERATORS};
use parser::TokenPair;
use objects::object::{ObjectType, Response};

use std::cmp::Ordering;

#[derive(Debug)]
pub struct OpereratorPlugin{}

pub static INSTANCE: OpereratorPlugin = OpereratorPlugin{};

impl Plugin for OpereratorPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let mut ret = PluginResponse::NoResponse;
      'oper_loop: for oper in OPERATORS.iter() { // TODO: Enum iteration
         'is_oper: loop {
            {
               let oper_str = oper.symbol;
               if oper_str.len() > 1 {
                  panic!("oper_str length != 1 (TODO THIS): {:?}", oper_str);
               }
               let peeked = match env.stream.peek_char() {
                  None => return PluginResponse::NoResponse,
                  Some(obj) => obj,
               };
               if peeked.source_val.to_string() != oper_str {
                  break 'is_oper
               }
            }
            env.stream.next();
            ret = PluginResponse::Response(Box::new(oper.clone()));
            break 'oper_loop;
         }
      }
      ret
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {
      if let ObjectType::Operator(oper) = (*token).obj_type(){
         let lhs = if oper.has_lhs{
                         Some(OpereratorPlugin::get_lhs(oper, env))
                   } else {
                         None
                   };

         let rhs = if oper.has_rhs{
                         Some(OpereratorPlugin::get_rhs(oper, env))
                   } else {
                         None
                   };

         match (oper.func)(lhs, rhs, env){
            Response::Return(obj) => env.universe.push(obj),
            Response::VoidReturn => {},
            _ => panic!("TODO: Unimplemented"),
         }
      } else {
         panic!("Bad ObjectType for OperatorPlugin::handle")
      }
   }
}

impl OpereratorPlugin{
   fn get_lhs(_: &Operator, env: &mut Environment) -> BoxedObj {
      unwrap!(env.universe.pop(), "get_lhs")
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> BoxedObj {
      let oper_priority = oper.priority;
      loop {
         match env.parser.next_object(env) {
            Ok(TokenPair(token, plugin)) => {
               let token_priority = match (*token).obj_type() {
                  ObjectType::Operator(oper) => oper.priority,
                  _ => 0
               };
               if oper_priority <= token_priority {
                  for x in token.source() {
                     env.stream.feed(Box::new(x));
                  }
                  break
               }
               plugin.handle(token, env);
            },
            Err(EOF) => 
         }
      }
      env.universe.stack.pop().unwrap()
   }
}














