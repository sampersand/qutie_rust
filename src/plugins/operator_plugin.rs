use plugins::plugin::Plugin;
use environment::Environment;
use plugins::NextObjectResult;
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
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      let mut ret = NextObjectResult::NoResponse;
      'oper_loop: for oper in OPERATORS.values() { // TODO: Enum iteration
         println!("oper: {:?}", oper);
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
            ret = NextObjectResult::Response(Box::new(oper.clone()));
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
   fn get_lhs(_: &Operator, env: &mut Environment) -> BoxedObj{
      unwrap!(env.universe.pop(), "get_lhs")
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> BoxedObj{
      let oper_priority = oper.priority;
      loop {
         println!("0: stream: {}, stack: {}", env.stream, env.universe);
         let TokenPair(token, plugin) = env.parser.next_object(env);
         println!("1: stream: {}, stack: {}", env.stream, env.universe);
         let token_priority = match (*token).obj_type() {
            ObjectType::Operator(oper) => oper.priority,
            _ => 0
         };
         // println!("2: stream: {}, stack: {}", env.stream, env.universe);
         if oper_priority <= token_priority {
            for x in token.source() {
               env.stream.feed(Box::new(x));
            }
            break
         }
         // println!("3: stream: {}, stack: {}", env.stream, env.universe);
         plugin.handle(token, env);
         // println!("4: stream: {}, stack: {}", env.stream, env.universe);
      }
      env.universe.stack.pop().unwrap()
   }
}














