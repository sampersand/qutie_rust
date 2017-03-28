use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;
use objects::symbol::Symbol;
use objects::operator::Operator;
use objects::operator;
use parser::TokenPair;
use objects::object::ObjType;

use result::{ObjError};

#[derive(Debug)]
pub struct OperatorPlugin;
pub static INSTANCE: &'static OperatorPlugin = &OperatorPlugin{};

impl Plugin for OperatorPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      use regex::Regex;
      lazy_static! {
         static ref ONLY_ALPHANUM_REGEX: Regex = Regex::new(r"^[a-zA-Z_0-9]+$").unwrap();
      }
      let lcls = env.universe.locals.clone();
      let glbls = env.universe.globals.clone();
      let operators: Vec<&Rc<Object>> = { /* this hsould become an iter */
         let mut tmp: Vec<&Rc<Object>> = vec![];
         for obj in lcls.values().chain(glbls.values()) {
            if let ObjType::Operator(oper) = obj.obj_type() {
               // tmp.push(rc!(*(&**obj as *const Operator)))
               tmp.push(obj)
            }
         };
         tmp.sort_by(|a, b| cast_as!(b, Operator).sigil.len().cmp(&cast_as!(a, Operator).sigil.len()));
         tmp
      };

      for oper in operators.iter() {
         let ref oper_str = cast_as!(oper, Operator).sigil;
         let mut oper_acc = String::new();

         for chr in oper_str.chars() {
            match env.stream.peek() {
               Some(c) if *c == chr => {
                  oper_acc.push(chr);
                  env.stream.next();
               },
               _ => {
                  env.stream.feed_back(rc!(Symbol::new(oper_acc.clone())));
                  oper_acc.clear();
                  break;
               }
            }
         }

         if oper_acc.len() == oper_str.len() {
            use plugins::symbol_plugin;
            if ONLY_ALPHANUM_REGEX.is_match(oper_acc.as_str()) && symbol_plugin::is_symbol_cont(peek!(env, '*')) {
               env.stream.feed_back(rc!(Symbol::new(oper_acc.clone())));
            } else {
               return PluginResponse::Response(Ok((*oper).clone()));
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

unsafe fn to_static<'a, T>(inp: &'a T) -> &'static T {
   use std::mem;
   mem::transmute::<&'a T, &'static T>(inp)
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
      let mut __was_transmuted = false;
      loop {
         let TokenPair(token, plugin) = cloned_env.next_object(env);
         match token {
            Ok(obj) => {
               /* __ */ unsafe {
                  if oper.sigil == "." {
                     if let ObjType::Operator(next_oper) = obj.obj_type() {
                        if next_oper.sigil == "=" {
                           assert!(!__was_transmuted);
                           use objects::symbol::Symbol;
                           use objects::universe::AccessType;
                           *oper = to_static(cast_as!(env.universe
                                      .get(rc!(Symbol::from(".=")),
                                           AccessType::NonStack)
                                      .unwrap(), Operator));
                           __was_transmuted = true;
                           continue;
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
                  env.stream.feed_back(obj);
                  break
               }
               plugin.handle(obj, env);
            },
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
         }
      }
      if __was_transmuted {
         let new_uni = Universe::new(None,
                                     Some(vec![env.universe.pop().unwrap(),
                                              env.universe.pop().unwrap()]),
                                     None,
                                     None);
         env.universe.push(rc!(new_uni))
      }
      match env.universe.pop() {
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      }
   }
}














