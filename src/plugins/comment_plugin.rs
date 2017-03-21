use env::Environment;
use objects::obj_rc::ObjRc;
use result::ObjError;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::{number_plugin};
use regex::Regex;

#[derive(Debug)]
pub struct CommentPlugin;

pub static INSTANCE: &'static CommentPlugin = &CommentPlugin{};


fn pre_handle_command(cmd: &str, args: &str, env: &mut Environment) {
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

impl CommentPlugin {
   fn parse_comment(comment: &str, env: &mut Environment) {
      lazy_static! {
         static ref COMMENT_REGEX: Regex = Regex::new(r"^\[(\w+)\((.*)\)\]$").unwrap();
      }
      if let Some(captures) = COMMENT_REGEX.captures(comment) {
         let cmd = captures.get(1).unwrap().as_str();
         let args = captures.get(2).unwrap().as_str();
         pre_handle_command(cmd, args, env);
      }
   }

   fn multi_line(env: &mut Environment) -> PluginResponse {
      PluginResponse::NoResponse
   }

   fn single_line(env: &mut Environment) -> PluginResponse{
      const LINE_START: char = '#';
      const LINE_ENDL: char = '\n';
      if LINE_START == peek_char!(env, EndOfFile => '_') {  /* `_` can't be LINE_START */
         let mut comment_acc = String::new();
         loop {
            env.stream.next();
            let peeked_char = peek_char!(env, EndOfFile => break);
            if LINE_ENDL == peeked_char {
               break
            }
            comment_acc.push(peeked_char);
         }
         CommentPlugin::parse_comment(comment_acc.as_str(), env);
         PluginResponse::Retry
      } else {
         PluginResponse::NoResponse
      }
   }
}
impl Plugin for CommentPlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match CommentPlugin::multi_line(env) {
         PluginResponse::NoResponse => CommentPlugin::single_line(env),
         other @ _ => other,
      }
   }
   fn handle(&self, _: ObjRc, _: &mut Environment) {
      unreachable!(); // we shouldn't be handling comments
   }
}














