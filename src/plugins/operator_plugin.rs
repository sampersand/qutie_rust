use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;
use objects::operator::Operator;
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
            let non_chr = match chr {
               '_' => 'a',
               _ => '_'
            };

            if chr == peek_char!(env, EndOfFile => non_chr) {
               oper_acc.push(chr);
               env.stream.next();
            } else {
               feed_back(env, oper_acc.as_str());
               oper_acc.clear();
               break;
            }
         }
         if oper_acc.len() == oper_str.len() {
            use plugins::symbol_plugin;
            if ONLY_ALPHANUM_REGEX.is_match(oper_acc.as_str()) && symbol_plugin::is_symbol_cont(peek_char!(env, EndOfFile => '*')) {
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
      match (*token).obj_type(){
         ObjType::Operator(oper) =>  {
            let lhs = match oper.has_lhs { 
               true => Some(OperatorPlugin::get_lhs(oper, env)),
               false => None,
            };

            let rhs = match oper.has_rhs {
               true => Some(OperatorPlugin::get_rhs(oper, env)),
               false => None 
            };

            oper.call_oper(lhs, rhs, env);
         },
         other @ _ => panic!("Bad ObjType for OperatorPlugin::handle: {:?}", other)
      }
   }
}

impl OperatorPlugin{
   fn get_lhs(oper: &Operator, env: &mut Environment) -> ObjRc {

      match env.universe.pop(){
         Ok(obj) => obj,
         Err(err) => panic!("Err with lhs of oper {:?}: {:?}", oper, err)
      }
   }

   fn get_rhs(oper: &Operator, env: &mut Environment) -> ObjRc {
      let oper_priority = oper.priority;
      let cloned_env = env.parser.clone();
      loop {
         let TokenPair(token, plugin) = cloned_env.next_object(env);
         match token {
            Ok(obj) => {
               let token_priority = match (*obj).obj_type() {
                  ObjType::Operator(oper) => oper.priority,
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














