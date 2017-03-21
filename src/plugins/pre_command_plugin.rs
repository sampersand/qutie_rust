use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError;

use objects::universe::Universe;
use parser::Parser;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{number_plugin, symbol_plugin, text_plugin,
              whitespace_plugin, universe_plugin, comment_plugin,
              default_plugin, operator_plugin};

use regex::Regex;

#[derive(Debug)]
pub struct PreCommandPlugin;

pub static INSTANCE: &'static PreCommandPlugin = &PreCommandPlugin{};

fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
   match cmd {
      "include" => {
         for pl in args.split(", ") {
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
      },
      "include_oper" => {
         for pl in args.split(", ") {
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
      }
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














