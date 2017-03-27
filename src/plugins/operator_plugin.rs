use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;
use objects::operator::Operator;
use objects::operator;
use parser::TokenPair;
use objects::object::ObjType;

use result::{ObjError};

#[derive(Debug)]
pub struct OperatorPlugin;
pub static INSTANCE: &'static OperatorPlugin = &OperatorPlugin{};

fn feed_back(env: &mut Environment, inp: &str) {
   let mut feed_stack = Universe::parse_str(inp);
   feed_stack.reverse();
   for single_chr in feed_stack {
      env.stream.feed(single_chr);
   }
}
impl Plugin for OperatorPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      use regex::Regex;
      lazy_static! {
         static ref ONLY_ALPHANUM_REGEX: Regex = Regex::new(r"^[a-zA-Z_0-9]+$").unwrap();
      }

      let operators: Vec<Operator> = { /* this hsould become an iter */
         let mut tmp = vec![];
         for oper in env.universe.locals.values().chain(env.universe.globals.values()) {
            if let ObjType::Operator(oper) = oper.obj_type() {
               tmp.push(oper.clone())
            }
         };
         tmp.sort_by(|a, b| b.sigil.len().cmp(&a.sigil.len()));
         tmp
      };

      for oper in operators.iter() {
         let oper_str = (*oper.sigil).clone();
         let mut oper_acc = String::new();

         for chr in oper_str.chars() {
            match env.stream.peek_char() {
               Some(c) if c == chr => {
                  oper_acc.push(chr);
                  env.stream.next();
               },
               _ => {
                  feed_back(env, oper_acc.as_str());
                  oper_acc.clear();
                  break;
               }
            }
         }

         if oper_acc.len() == oper_str.len() {
            use plugins::symbol_plugin;
            if ONLY_ALPHANUM_REGEX.is_match(oper_acc.as_str()) && symbol_plugin::is_symbol_cont(peek_char!(env, '*')) {
               feed_back(env, oper_acc.as_str());
            } else {
               return ok_rc!(RESP; oper.clone());
            }
         } else {
            assert_eq!(oper_acc.len(), 0);
         }
      }
      PluginResponse::NoResponse
   }
   fn handle(&self, token: ObjRc, env: &mut Environment) {
      if let ObjType::Operator(mut oper) = token.obj_type() {
         let ref mut oper = oper;
         let lhs = if oper.has_lhs { 
                      Some(OperatorPlugin::get_lhs(oper, env))
                   } else {
                      None
                   };
         let rhs = if oper.has_rhs {
                      Some(OperatorPlugin::get_rhs(oper, env))
                   } else {
                      None 
                   };
         oper.call_oper(lhs, rhs, env);
      } else {
         panic!("Bad ObjType for OperatorPlugin::handle")
      }
   }
}

impl OperatorPlugin{
   fn get_lhs(oper: &mut &Operator, env: &mut Environment) -> ObjRc {

      match env.universe.pop(){
         Ok(obj) => obj,
         Err(err) => panic!("Err with lhs of oper {:?}: {:?}", oper, err)
      }
   }

   fn get_rhs(oper: &mut &Operator, env: &mut Environment) -> ObjRc {
      let oper_priority = oper.priority;
      let cloned_env = env.parser.clone();
      loop {
         let TokenPair(token, plugin) = cloned_env.next_object(env);
         match token {
            Ok(obj) => {
               unsafe {
                  if *oper.sigil == "." {
                     if let ObjType::Operator(next_oper) = obj.obj_type() {
                        if *next_oper.sigil == "=" {
                           *oper = &*operator::SET_OPER;
                        }
                     }
                  }
               }

               let token_priority = match (*obj).obj_type() {
                  ObjType::Operator(o) => o.priority,
                  _ => 0
               };
               // maybe instead of source, we just use a double pointer? but that'd require changing all other plugins
               // or we jsut "rebase" inside environment
               if oper_priority <= token_priority {
                  let mut src = obj.source();
                  src.reverse();
                  for x in src {
                     env.stream.feed(rc!(x));
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














