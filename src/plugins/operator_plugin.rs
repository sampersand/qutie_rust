use plugins::plugin::Plugin;
use environment::Environment;
use plugins::NextObjectResult;
use plugins::NextObjectResult::{NoResponse, Response};
use objects::boxed_obj::BoxedObj;
use objects::object::{Object, FunctionError};
use objects::null::Null;
use objects::operator::{Operator, OPERATORS};
use parser::TokenPair;
use objects::object::ObjectType;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct OpereratorPlugin{}

pub static INSTANCE: OpereratorPlugin = OpereratorPlugin{};

impl Plugin for OpereratorPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      let mut ret = NoResponse;
      'oper_loop: for oper in OPERATORS.values() { // TODO: Enum iteration
         let mut i = 0;
         'is_oper: loop {
            {
               let oper_str = oper.symbol;
               let peeked = env.stream.peek_char_amnt(oper_str.len());
               while i < oper_str.len() { // TODO: FOR LOOPS
                  if peeked[i].source_val.to_string() != oper_str[0..1]{
                     break 'is_oper;
                  }
                  i += 1;
               }
            }
            while 0 < i { // TODO: FOR LOOPS
               env.stream.next();
               i -= 1;
            }
            ret = Response(Box::new(oper.clone()));
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
            Ok(obj) => env.universe.push(obj),
            Err(err) => match err {
               FunctionError::NoResponse => panic!("TODO: Unimplemented"),
               FunctionError::VoidResponse => {}
            }
         }
      } else {
         panic!("Bad ObjectType for OperatorPlugin::handle")
      }
   }
}

impl OpereratorPlugin{
   fn get_lhs(_: &Operator, env: &mut Environment) -> BoxedObj{
      unwrap!(env.universe.pop(), "get_lhs")
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> BoxedObj{
      let mut ret: Vec<BoxedObj> = vec![];
      let oper_priority = oper.priority;
      let mut delete_me = 0;
      loop {
         let TokenPair(token, plugin) = env.parser.next_object(env);
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
         ret.push(env.universe.stack.pop().unwrap());
         if delete_me > 20 { panic!("delete_me > 20"); } else {delete_me += 1}
      }
      ret.pop().unwrap()
   }
}














