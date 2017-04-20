use env::Environment;
use std::rc::Rc;
use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::symbol_plugin;
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
                  let oper = cast_as!(CL; obj, Operator);
                  let sigil = oper.sigil.clone();
                  tmp.push((oper, sigil))
               }
            };
            tmp.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            tmp
         };

      for &(ref oper, ref sigil) in operators.iter() {
         assert_debug!(eq; &oper.sigil, sigil);
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
               // if !env.stream.is_empty() && index == sigil.len() - 1 {
               //    if ONLY_ALPHANUM_REGEX.is_match(sigil) {
               //       if symbol_plugin::is_symbol_cont(env.stream.peek().unwrap().chr) {
               //          println!("INb4: {}, {:?}", sigil, env.stream);
               //          env.stream.feed(' ');
               //          env.stream.feed_back(oper.clone());
               //          println!("INl8: {}, {:?}", sigil, env.stream);
               //       }
               //    }
               // }
               break
            } else if index == sigil.len() -1 {
               return resp_ok!(oper.clone())
            }
         }
      }
      PluginResponse::NoResponse
   }

   fn handle(&self, input: ObjRc, env: &mut Environment) {
      assert_debug!(is_a; input, Operator);

      let ref mut oper = cast_as!(input, Operator);
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

impl OperatorPlugin{
   fn get_lhs(oper: &mut Rc<Operator>, env: &mut Environment) -> ObjRc {
      if cfg!(debug = "true") {
         let err_msg = "Can't get lhs of operator:".to_string() + oper.to_string().as_str();
         env.universe.pop().expect(err_msg.as_str())
      } else {
         env.universe.pop().expect("Can't get lhs of operator")
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
                           *oper = cast_as!(env.universe.get(new_obj!(SYM_STATIC, ".="), AccessType::NonStack).unwrap(), Operator);
                           // panic!("TODO: .=");
                           let stack_len = env.universe.stack.len() - 2;
                           env.universe.stack.remove(stack_len);
                           let new_oper = env.universe.get(new_obj!(SYM_STATIC, ".="), AccessType::NonStack).unwrap().clone();
                           env.universe.stack.insert(stack_len, new_oper);
                           __was_transmuted = true;
                           continue;
                        }
                     }
                  }
               }

               let obj_priority = 
                  if obj.is_a(ObjType::Operator) {
                     cast_as!(CL; obj, Operator).priority
                  } else {
                     0
                  };
               // maybe instead of feed_back, we just use a double pointer? but that'd require changing all other plugins
               // or we just "rebase" inside environment
               if oper_priority <= obj_priority {
                  env.stream.feed_back(obj);
                  break
               }
               plugin.handle(obj, env);
            },
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Don't know how to handle ObjError: {:?}", err)
         }
      }
// i stoped working here
      if __was_transmuted { /* AKA was it turned from `.` into `.=` */
         let key = env.universe.pop().expect("Error with `.=`: `key` couldn't be found");
         let val = env.universe.pop().expect("Error with `.=`: `val` couldn't be found");
         let new_uni = Universe::new_rc(None, Some(vec![key, val]), None, None);
         env.universe.push(new_uni)
      }

      for x in uni_start_len..(env.universe.stack.len()-1) {
         env.stream.feed_back(env.universe.stack.pop().unwrap());
      }

      let ret = env.universe.pop().expect("Can't find the return value for rhs side!");
      let last_val = env.universe.pop().expect("Can't find the operator");
      assert_eq!(**oper, *cast_as!(last_val, Operator)); // remove the oper we pushed on stack
      ret
   }
}














