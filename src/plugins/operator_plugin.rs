use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use plugins::next_object_result::NextObjectResult::{NoResponse, Response};
use objects::boxed_obj::BoxedObj;
use objects::operator::Operator;
use parser::TokenPair;
use objects::object::ObjectType;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct SymbolPlugin{}

pub static INSTANCE: SymbolPlugin = SymbolPlugin{};

impl Plugin for SymbolPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      let mut ret = NoResponse;
      'oper_loop: for oper in vec![Operator::Equals] { // TODO: Enum iteration
         let mut i = 0;
         'is_oper: loop {
            {
               let oper_str = oper.get_value();
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
            ret = Response(Box::new(oper));
            break 'oper_loop;
         }
      }
      ret
   }
   fn handle(&self, token: BoxedObj, env: &mut Environment) {
      if let ObjectType::Operator(oper) = (*token).obj_type() {
         let lhs_vars = SymbolPlugin::get_lhs(oper, env);
         let rhs_vars = SymbolPlugin::get_rhs(oper, env);
         println!("{:?}", rhs_vars);
      } else {
         panic!("Bad!");
      }
   }
}
impl SymbolPlugin{
   fn get_lhs(_: &Operator, env: &mut Environment) -> Vec<BoxedObj>{
      vec!(match env.universe.pop(){Some(e)=>e,None=>panic!("bad, no ")})
   }
   fn get_rhs(oper: &Operator, env: &mut Environment) -> Vec<BoxedObj>{
      let mut ret: Vec<BoxedObj> = vec![];
      let oper_priority = oper.priority();
      loop {
         let TokenPair(token, plugin) = env.parser.next_object(env);
         let token_priority = match (*token).obj_type() {
            ObjectType::Operator(oper) => oper.priority(),
            _ => 0
         };
         if token_priority <= oper_priority {
            env.stream.feed(token);
            break
         }
         plugin.handle(token, env);
      }
      ret
   }
   // fn get_rhs(oper: &Operator, env: &mut Environment) -> Vec<BoxedObj>{
   //    let mut ret: Vec<BoxedObj> = vec![];
   //    let oper_priority = oper.priority();
   //    let mut rhs = env.universe.spawn_clone_stack();
   //    loop {
   //       let token_priority = {
   //          let stream_clone = &mut env.stream.clone();
   //          let env_fork = &mut Environment::new(stream_clone, &mut rhs, env.parser);
   //          let TokenPair(token, plugin) = env.parser.next_object(env_fork);
   //          match (*token).obj_type() {
   //             ObjectType::Operator(oper) => oper.priority(),
   //             _ => 0
   //          }
   //       };
   //       if token_priority <= oper_priority { break }
   //       let TokenPair(token, plugin) = {
   //          let env_fork = &mut Environment::new(env.stream, &mut rhs, env.parser);
   //          env.parser.next_object(env_fork)
   //       };
   //       plugin.handle(token, env);
   //    }
   //    ret
   // }
}














