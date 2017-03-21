use env::Environment;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use std::rc::Rc;
use result::ObjError;

use objects::operator;
use objects::object::Object;
use objects::universe::Universe;
use parser::Parser;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::universe::AccessType;
use plugins::{number_plugin, symbol_plugin, text_plugin,
              whitespace_plugin, universe_plugin, comment_plugin,
              default_plugin, operator_plugin};
use objects::symbol::Symbol;

use regex::Regex;

#[derive(Debug)]
pub struct PreCommandPlugin;

pub static INSTANCE: &'static PreCommandPlugin = &PreCommandPlugin{};

fn add_plugin(pl: &str, env: &mut Environment) {
   env.parser.add_plugin(match pl {
      "Number" => number_plugin::INSTANCE,
      "Symbol" => symbol_plugin::INSTANCE,
      "Text" => text_plugin::INSTANCE,
      "Whitespace" => whitespace_plugin::INSTANCE,
      "Universe" => universe_plugin::INSTANCE,
      "Comment" => comment_plugin::INSTANCE,
      "Default" => default_plugin::INSTANCE,
      "Operator" => operator_plugin::INSTANCE,
      other @ _ => panic!("Unknown plugin to include: {:?}", pl)
   })
}
fn add_oper(oper: &str, env: &mut Environment) {
   let opers = operator::operators();
   let key = rc!(Symbol::new(oper.to_string()));
   let value = match opers.get(&ObjRcWrapper(key.clone())) {
      Some(oper) => oper,
      None => panic!("No operator {:?} found", oper),
   };
   env.universe.set(key, value.clone(), AccessType::Locals);
}

fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
   match cmd {
      "include" => for pl in args.split(", "){ add_plugin(pl, env) },
      "include_oper" => for oper in args.split(", "){ add_oper(oper, env) },
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














