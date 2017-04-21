use env::Environment;
use std::rc::Rc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::symbol::Symbol;
use result::ObjError::EndOfFile;

#[derive(Debug)]
pub struct SymbolPlugin;

pub static INSTANCE: &'static SymbolPlugin = &SymbolPlugin{};

use stream::StreamChar;
fn is_symbol_start(inp: StreamChar) -> bool {
   inp.is_alphabetic() || inp == '_'
}
pub fn is_symbol_cont(inp: StreamChar) -> bool {
   inp.is_alphanumeric() || inp == '_'
}

impl Plugin for SymbolPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      match env.stream.peek() {
         Some(ref c) if is_symbol_start(c.chr) => {},
         _ => return PluginResponse::NoResponse
      }

      let mut symbol_acc: String = String::new();
      assert_debug!(is_symbol_start(env.stream.peek().expect("couldn't find start of stream").chr));

      while let Some(ref mut c) = env.stream.peek() {
         if is_symbol_cont(c.chr) {
            symbol_acc.push(c.take());
         } else {
            break
         }
      }

      assert_debug!(!symbol_acc.is_empty());
      let sym = Symbol::from(symbol_acc).to_rc();
      resp_ok!(sym)
   }
}














