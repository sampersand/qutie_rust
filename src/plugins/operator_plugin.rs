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
use objects::object::{ObjWrapper, ObjType};

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

      let operators = 
         {
            let mut tmp = vec!();
            for obj in env.universe.locals.values().chain(env.universe.globals.values()) {
               if obj.is_a(ObjType::Operator) {
                  tmp.push(cast_as!(CL; obj, Operator))
               }
            };
            tmp.sort_by(|a, b| b.sigil.len().cmp(&a.sigil.len()));
            tmp
         };

      for oper in operators.iter() {
         let ref sigil = oper.sigil;
         for (index, chr) in sigil.chars().enumerate() {
            let do_stop = 
               match env.stream.peek() {
                  Some(ref mut c) if c.chr == chr => 
                     {
                        c.take();
                        false
                     },
                  _ => true
               };
            if do_stop {
               for i in 0..index {
                  env.stream.feed(sigil.chars().nth(index - i - 1).unwrap())
               }
               break
            } else if index == sigil.len() -1 {
               return PluginResponse::Response(Ok(oper.clone()))
            }
         }
      }
      PluginResponse::NoResponse
   }

   fn handle(&self, token: ObjRc, env: &mut Environment) {
      if !token.is_a(ObjType::Operator) {
         panic!("Bad OldObjType for OperatorPlugin::handle")
      }

      let ref mut oper = cast_as!(token, Operator);
      let lhs = 
         if oper.has_lhs { 
            Some(OperatorPlugin::get_lhs(oper, env))
         } else {
            None
         };
      let rhs = 
         if oper.has_rhs {
            Some(OperatorPlugin::get_rhs(oper, env))
         } else {
            None 
         };
      oper.call_oper(lhs, rhs, env);
   }
}

unsafe fn to_static<'a, T>(inp: &'a T) -> &'static T {
   use std::mem;
   mem::transmute::<&'a T, &'static T>(inp)
}

impl OperatorPlugin{
   fn get_lhs(oper: &mut Rc<Operator>, env: &mut Environment) -> ObjRc {
      if let Ok(obj) = env.universe.pop() {
         obj
      } else {
         panic!("Err with lhs of oper ({:?})!", oper)
      }
   }

   fn get_rhs(oper: &mut Rc<Operator>, env: &mut Environment) -> ObjRc {
      let mut __was_transmuted = false;
      env.universe.push(oper.clone());
      let uni_start_len = env.universe.stack.len();
      loop {
         let TokenPair(token, plugin) = env.parser.clone().next_object(env);
         let oper_priority = oper.priority;
         match token {
            Ok(obj) => {
               unsafe {
                  if oper.sigil == "." {
                     if obj.is_a(ObjType::Operator) {
                        let next_oper = cast_as!(CL; obj, Operator);
                        if next_oper.sigil == "=" {
                           assert!(!__was_transmuted);
                           use objects::symbol::Symbol;
                           use objects::universe::AccessType;
                           // *oper = to_static(cast_as!(env.universe.get(rc!(Symbol::from(".=")), AccessType::NonStack).unwrap(), Operator));
                           panic!("TODO: .=");
                           let stack_len = env.universe.stack.len() - 2;
                           env.universe.stack.remove(stack_len);
                           let new_oper = env.universe.get(rc!(Symbol::from(".=")), AccessType::NonStack).unwrap().clone();
                           env.universe.stack.insert(stack_len, new_oper);
                           __was_transmuted = true;
                           continue;
                        }
                     }
                  }
               }

               let token_priority = 
                  if obj.is_a(ObjType::Operator) {
                     cast_as!(CL; obj, Operator).priority
                  } else {
                     0
                  };
               // maybe instead of feed_back, we just use a double pointer? but that'd require changing all other plugins
               // or we just "rebase" inside environment
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

      if __was_transmuted { /* AKA was it turned from `.` into `.=` */
         let key = env.universe.pop().unwrap();
         let val = env.universe.pop().unwrap();
         let new_uni = Universe::new(None, Some(vec![key, val]), None, None);
         env.universe.push(rc!(new_uni))
      }

      for x in uni_start_len..(env.universe.stack.len()-1) {
         env.stream.feed_back(env.universe.stack.pop().unwrap());
      }

      let ret = env.universe.pop().expect("Can't find the return value for rhs side!");
      assert_eq!(**oper, *cast_as!(env.universe.pop().unwrap(), Operator)); // remove the oper we pushed on stack
      ret
   }
}














