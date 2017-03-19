use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;
use objects::operator::{Operator, OPERATORS};
use parser::TokenPair;
use objects::object::ObjType;

use result::{ObjError};

#[derive(Debug)]
pub struct OpereratorPlugin;
pub static INSTANCE: OpereratorPlugin = OpereratorPlugin{};

impl Plugin for OpereratorPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let mut ret = PluginResponse::NoResponse;
      'oper_loop: for oper in OPERATORS.iter() { // TODO: Enum iteration
         'is_oper: loop {
            {
               let oper_str = oper.sigil;
               if oper_str.len() > 1 {
                  panic!("oper_str length != 1 (TODO THIS): {:?}", oper_str);
               }
               let peeked = match env.stream.peek_char() {
                  Ok(obj) => obj,
                  Err(ObjError::EndOfFile) => break 'is_oper,
                  Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
               };
               if peeked.char_val.to_string() != oper_str {
                  break 'is_oper
               }
            }
            let _next_char = env.stream.next();
            ret = PluginResponse::Response(Ok(Rc::new(oper.clone())));
            break 'oper_loop;
         }
      }
      ret
   }
   fn handle(&self, token: ObjRc, env: &mut Environment) {
      match (*token).obj_type(){
         ObjType::Operator(oper) =>  {
            let lhs = match oper.has_lhs { 
               true => Some(OpereratorPlugin::get_lhs(env)),
               false => None,
            };

            let rhs = match oper.has_rhs {
               true => Some(OpereratorPlugin::get_rhs(env)),
               false => None 
            };

            oper.call_oper(lhs, rhs, env);
         },
         other @ _ => panic!("Bad ObjType for OperatorPlugin::handle: {:?}", other)
      }
   }
}

impl OpereratorPlugin{
   fn get_lhs(oper: &Operator, env: &mut Environment) -> ObjRc {
      match env.universe.pop(){
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> ObjRc {
      let oper_priority = oper.priority;
      loop {
         let TokenPair(token, plugin) = env.parser.next_object(env);
         match token {
            Ok(obj) => {
               let token_priority = match (*obj).obj_type() {
                  ObjType::Operator(oper) => oper.priority,
                  _ => 0
               };
               // maybe instead of source, we just use a double pointer? but that'd require changing all other plugins
               if oper_priority <= token_priority {
                  for x in obj.source() {
                     env.stream.feed(Rc::new(x));
                  }
                  break
               }
               plugin.handle(obj, env);
            },
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
         }
      }
      match env.universe.pop() {
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }
}














