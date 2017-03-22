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
use plugins::{operator_plugin, plugins};
use objects::symbol::Symbol;
use objects::object::ObjType;

use regex::Regex;

#[derive(Debug)]
pub struct PreCommandPlugin;

pub static INSTANCE: &'static PreCommandPlugin = &PreCommandPlugin{};

fn include(inp: &str, env: &mut Environment) {
   let key = rc!(Symbol::new(inp.to_string()));
   let ref wrapped_key = ObjRcWrapper(key.clone());

   if let Some(plugin) = plugins().get(wrapped_key) {
      env.parser.add_plugin(*plugin);
   } else if let Some(oper) = operator::operators().get(wrapped_key) {
      env.universe.set(key, oper.clone(), AccessType::Locals);
   } else if let Some(oper) = builtins::builtins().get(wrapped_key) {
      env.universe.set(key, oper.clone(), AccessType::Locals);
   } else { 
      panic!("Bad include input: {:?}", inp)
   }
}

fn exclude(inp: &str, env: &mut Environment) {
   let key = rc!(Symbol::new(inp.to_string()));
   let ref wrapped_key = ObjRcWrapper(key.clone());

   if let Some(plugin) = plugins().get(wrapped_key) {
      env.parser.del_plugin(*plugin);
   } else if let Some(oper) = operator::operators().get(wrapped_key) {
      env.universe.del(key, AccessType::Locals);
   } else if let Some(oper) = builtins::builtins().get(wrapped_key) {
      env.universe.del(key, AccessType::Locals);
   } else {
      panic!("Bad exclude input: {:?}", inp)
   }
}

fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
   match cmd {
      "include" => for to_include in args.split(", "){ include(to_include, env) },
      "exclude" => for to_exclude in args.split(", "){ exclude(to_exclude, env) },
      other @ _ => panic!("Unknown pre-command {:?}", cmd)
   }
}


impl Plugin for PreCommandPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      lazy_static! {
         static ref COMMENT_REGEX: Regex = Regex::new(r"^\[(\w+)\((.*)\)\]\s*(?:#\s*)?$").unwrap();
      }
      const CMD_START: char = '#';
      const CMD_END: char = '\n';

      if CMD_START != peek_char!(env, EndOfFile => '_') {  /* `_` can't be CMD_START */
         return PluginResponse::NoResponse;
      }

      let mut cmd_acc = String::new();

      loop {
         env.stream.next();
         let peeked_char = peek_char!(env, EndOfFile => break);
         if CMD_END == peeked_char { break }
         cmd_acc.push(peeked_char);
      }
      env.stream.next(); // peek the endl

      if let Some(captures) = COMMENT_REGEX.captures(cmd_acc.as_str()) {
         let cmd = captures.get(1).unwrap().as_str();
         let args = captures.get(2).unwrap().as_str();
         pre_handle_command(cmd, args, env);
      } else {
         panic!("Bad cmd string: {:?}", cmd_acc)
      }
      PluginResponse::Retry
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling pre_commands
   }
}














