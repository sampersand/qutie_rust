use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::boxed_obj::BoxedObj;
use objects::operator::{Operator, OPERATORS};
use parser::TokenPair;
use objects::object::ObjectType;

use result::{ObjResult, ObjError};

#[derive(Debug)]
pub struct OpereratorPlugin;
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
                  Ok(obj) => obj,
                  Err(ObjError::EndOfFile) => break 'is_oper,
                  Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
               };
               if peeked.source_val.to_string() != oper_str {
                  break 'is_oper
               }
            }
            let _next_char = env.stream.next();
            ret = PluginResponse::Response(Ok(Box::new(oper.clone())));
            break 'oper_loop;
         }
      }
      ret
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {
      match (*token).obj_type(){
         ObjectType::Operator(oper) =>  {
            let lhs = if oper.has_lhs { 
                  Some(OpereratorPlugin::get_lhs(oper, env))
               } else {
                  None
               };

            let rhs = if oper.has_rhs {
                  Some(OpereratorPlugin::get_rhs(oper, env))
               } else {
                  None
               };
            match (oper.func)(lhs, rhs, env) {
               Ok(obj) => env.universe.push(obj),
               Err(ObjError::NoResultDontFail) => {},
               Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
            }
         },
         other @ _ => panic!("Bad ObjectType for OperatorPlugin::handle: {:?}", other)
      }
   }
}

impl OpereratorPlugin{
   fn get_lhs(_: &Operator, env: &mut Environment) -> BoxedObj {
      match env.universe.pop(){
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> BoxedObj {
      let oper_priority = oper.priority;
      loop {
         let TokenPair(token, plugin) = env.parser.next_object(env);
         match token {
            Ok(obj) => {
               let token_priority = match (*obj).obj_type() {
                  ObjectType::Operator(oper) => oper.priority,
                  _ => 0
               };
               if oper_priority <= token_priority {
                  for x in obj.source() {
                     env.stream.feed(Box::new(x));
                  }
                  break
               }
               plugin.handle(obj, env);
            },
            Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
         }
      }
      match env.universe.pop() {
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }
}














