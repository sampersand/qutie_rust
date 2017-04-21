use env::Environment;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use std::rc::Rc;
use result::ObjError;

use objects::operator;
use builtins;

use objects::object::Object;
use objects::universe::Universe;
use parser::Parser;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::universe::AccessType;
use plugins::{operator_plugin};
use plugins;

use objects::symbol::Symbol;

use regex::Regex;

#[derive(Debug)]
pub struct PreCommandPlugin;

pub static INSTANCE: &'static PreCommandPlugin = &PreCommandPlugin{};

fn include(inp: &str, env: &mut Environment, access_type: AccessType) {
   match inp {
      "*Plugins" => {
         let plugin_map = plugins::plugins();
         for plugin in plugins::plugin_order() {
            env.parser.add_plugin(*plugin_map.get(&plugin).expect("bad pre command plugin"));
         }
      },
      "*Builtins" => 
         for (ObjRcWrapper(key), val) in builtins::builtins() {
            env.universe.set(key, val, access_type);
         },
      "*Operators" =>
         for (ObjRcWrapper(key), val) in operator::operators(){
            env.universe.set(key, val, access_type);
         },
      _ => {
         let key = new_obj!(SYM, inp.to_string());
         let ref wrapped_key = ObjRcWrapper(key.clone());
         if let Some(plugin) = plugins::plugins().get(wrapped_key) {
            env.parser.add_plugin(*plugin);
         } else if let Some(oper) = operator::operators().get(wrapped_key) {
            env.universe.set(key, oper.clone(), access_type);
         } else if let Some(oper) = builtins::builtins().get(wrapped_key) {
            env.universe.set(key, oper.clone(), access_type);
         } else { 
            panic!("Bad include input: {:?}", inp)
         }
      }
   }
}

fn exclude(inp: &str, env: &mut Environment, access_type: AccessType) {
   let key = new_obj!(SYM, inp.to_string());
   let ref wrapped_key = ObjRcWrapper(key.clone());
   if let Some(plugin) = plugins::plugins().get(wrapped_key) {
      env.parser.del_plugin(*plugin);
   } else if let Some(oper) = operator::operators().get(wrapped_key) {
      env.universe.del(key, access_type);
   } else if let Some(oper) = builtins::builtins().get(wrapped_key) {
      env.universe.del(key, access_type);
   } else {
      panic!("Bad exclude input: {:?}", inp)
   }
}

fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
   let split_args = args.split(", ");
   match cmd {
      "include" => for to_include in split_args{ include(to_include, env, AccessType::Locals) },
      "exclude" => for to_exclude in split_args{ exclude(to_exclude, env, AccessType::Locals) },
      "include_glbl" => for to_include in split_args{ include(to_include, env, AccessType::Globals) },
      "exclude_glbl" => for to_exclude in split_args{ exclude(to_exclude, env, AccessType::Globals) },
      // "is_included" => for to_include in split_args{ include(to_include, env, AccessType::Globals) },

      other @ _ => panic!("Unknown pre-command {:?}", cmd)
   }
}


impl Plugin for PreCommandPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      lazy_static! {
         static ref CMD_REGEX: Regex = Regex::new(r"\[(\w+)\((.*)\)\]").expect("err with CMD_REGEX");
      }
      const CMD_START: char = '#';
      const CMD_END: char = ']';

      match env.stream.peek() {
         Some(ref obj) if obj.chr == CMD_START => {},
         _ => return PluginResponse::NoResponse
      }

      let mut cmd_acc = String::new();

      while let Some(mut obj) = env.stream.peek() {
         let peeked = obj.take();
         cmd_acc.push(peeked);
         if CMD_END == peeked { break }
      }

      if let Some(captures) = CMD_REGEX.captures(cmd_acc.as_str()) {
         let cmd = captures.get(1).unwrap().as_str();
         let args = captures.get(2).unwrap().as_str();
         pre_handle_command(cmd, args, env);
      } else {
         panic!("Bad cmd string: {:?}", cmd_acc)
      }
      PluginResponse::Retry
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling pre-commands
   }
}














