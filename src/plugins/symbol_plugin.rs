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

fn is_symbol_start(inp: char) -> bool {
   inp.is_alphabetic() || inp == '_'
}
pub fn is_symbol_cont(inp: char) -> bool {
   inp.is_alphanumeric() || inp == '_'
}

impl Plugin for SymbolPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      peek!(env, is_symbol_start, return NoResponse);

      let mut symbol_acc: String = String::new();

      while let Some(c) = env.stream.peek() {
         if !is_symbol_cont(*c) { break }
         symbol_acc.push(*c);
         assert_next_eq!(*c, env)
      }

      assert!(symbol_acc.len() > 0);
      let sym = Symbol::new(symbol_acc);
      Response(ok_rc!(sym))
   }
}














