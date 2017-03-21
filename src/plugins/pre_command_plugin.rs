use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError;

use objects::universe::Universe;
use parser::Parser;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{number_plugin};

use regex::Regex;

#[derive(Debug)]
pub struct PreCommandPlugin;

pub static INSTANCE: &'static PreCommandPlugin = &PreCommandPlugin{};

#[derive(Debug)]
pub struct MutEnvironment<'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'a mut Parser,//&'a mut Parser<'a>,
}

fn pre_handle_command(cmd: &str, args: &str, env: &mut MutEnvironment) {
   match cmd {
      "include" => { 
         env.parser.add_plugin(match args {
            "Number" => number_plugin::INSTANCE,
            other @ _ => panic!("Unknown include {:?}", args)
         });
      },
      other @ _ => panic!("Unknown pre-command {:?}", cmd)
   }
}


impl PreCommandPlugin {
   pub fn next_obj_mut(&self, env: &mut MutEnvironment) -> PluginResponse {
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
      }
      PluginResponse::Retry
   } /* end next_obj_mut */
}

impl Plugin for PreCommandPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      panic!("Don't use PreCommandPlugin.next_object, use PreCommandPlugin.next_obj_mut")
   }

   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling pre_commands
   }
}














