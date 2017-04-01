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
use objects::object::OldObjType;

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
            if let OldObjType::Operator(oper) = obj.old_obj_type() {
               tmp.push(obj)
            }
         };
         tmp.sort_by(|a, b| old_cast_as!(b, Operator).sigil.len().cmp(&old_cast_as!(a, Operator).sigil.len()));
         tmp
      };
      for oper in operators.iter() {
         let ref sigil = old_cast_as!(oper, Operator).sigil;
         for (index, chr) in sigil.chars().enumerate() {
            let do_stop = match env.stream.peek() {
                             Some(ref mut c) if c.chr == chr => { c.take(); false },
                             _ => true
                          };
            if do_stop {
               for i in 0..index {
                  env.stream.feed(sigil.chars().nth(index - i - 1).unwrap())
               }
               break
            } else if index == sigil.len() -1 {
               return PluginResponse::Response(Ok((*oper).clone()))
            }
         }
      }
      PluginResponse::NoResponse
   }

   fn handle(&self, token: ObjRc, env: &mut Environment) {

      if let OldObjType::Operator(mut oper) = token.old_obj_type() {
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
         panic!("Bad OldObjType for OperatorPlugin::handle")
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
      let cloned_parser = env.parser.clone();
      let mut __was_transmuted = false;
      env.universe.push(rc!(oper.clone()));
      let uni_len = env.universe.stack.len();
      loop {
         let TokenPair(token, plugin) = cloned_parser.next_object(env);
         let oper_priority = oper.priority;
         match token {
            Ok(obj) => {
               /* __ */ unsafe {
                  if oper.sigil == "." {
                     if let OldObjType::Operator(next_oper) = obj.old_obj_type() {
                        if next_oper.sigil == "=" {
                           assert!(!__was_transmuted);
                           use objects::symbol::Symbol;
                           use objects::universe::AccessType;
                           *oper = to_static(old_cast_as!(env.universe.get(rc!(Symbol::from(".=")), AccessType::NonStack).unwrap(), Operator));

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

               let token_priority = match (*obj).old_obj_type() {
                  OldObjType::Operator(o) => o.priority,
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
                                     None); // no parent needed
         env.universe.push(rc!(new_uni))
      }
      for x in uni_len..(env.universe.stack.len()-1) {
         env.stream.feed_back(env.universe.stack.pop().unwrap());
      }
      let ret = match env.universe.pop() {
         Ok(obj) => obj,
         Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
      };
      // println!("{:?} ? {:?} | {:?}", oper, env.universe, env.stream);
      assert_eq!(**oper, *old_cast_as!(env.universe.pop().unwrap(), Operator));
      ret
   }
}














