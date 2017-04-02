use env::Environment;
use std::rc::Rc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};
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
         _ => return NoResponse
      }

      let mut symbol_acc: String = String::new();

      while let Some(ref mut c) = env.stream.peek() {
         if !is_symbol_cont(c.chr) { break }
         symbol_acc.push(c.take());
      }

      assert!(0 < symbol_acc.len());
      let sym = Symbol::new(symbol_acc.as_str());
      Response(ok_rc!(sym))
   }
}














