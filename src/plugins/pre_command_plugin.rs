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

fn add_plugin(pl: &str, env: &mut Environment) {
   let plugins = plugins();
   if pl == "ALL" {
      for (_, val) in plugins {
         env.parser.add_plugin(val);
      }
      return;
   }
   let key = rc!(Symbol::new(pl.to_string()));
   let value = match plugins.get(&ObjRcWrapper(key.clone())) {
      Some(pl) => pl,
      None => panic!("No plugin {:?} found", pl),
   };
   env.parser.add_plugin(*value);
}

const INSERT_ACCESSTYPE: AccessType = AccessType::Locals;

fn add_oper(oper: &str, env: &mut Environment) {
   let opers = operator::operators();
   if !env.parser.has_plugin(operator_plugin::INSTANCE) {
      env.parser.add_plugin(operator_plugin::INSTANCE);
   }
   if oper == "ALL" {
      for (key, val) in opers {
         env.universe.set(key.0, val.clone(), INSERT_ACCESSTYPE);
      }
      return;
   }
   let key = rc!(Symbol::new(oper.to_string()));
   let value = match opers.get(&ObjRcWrapper(key.clone())) {
      Some(oper) => oper,
      None => panic!("No operator {:?} found", oper),
   };
   env.universe.set(key, value.clone(), INSERT_ACCESSTYPE);
}
fn add_builtin(builtin: &str, env: &mut Environment) {
   let builtins = builtins::builtins();
   if builtin == "ALL" {
      for (key, val) in builtins {
         env.universe.set(key.0, val.clone(), INSERT_ACCESSTYPE);
      }
      return;
   }
   let key = rc!(Symbol::new(builtin.to_string()));
   let value = match builtins.get(&ObjRcWrapper(key.clone())) {
      Some(builtin) => builtin,
      None => panic!("No builtin {:?} found", builtin),
   };
   env.universe.set(key, value.clone(), INSERT_ACCESSTYPE);
}


fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
   match cmd {
      "include_plugin" => for pl in args.split(", "){ add_plugin(pl, env) },
      "include_oper" => for oper in args.split(", "){ add_oper(oper, env) },
      "include_builtin" => for builtin in args.split(", "){ add_builtin(builtin, env) },
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














