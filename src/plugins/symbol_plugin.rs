use env::Environment;
use std::rc::Rc;
use parser::Parser;
use objects::universe::Universe;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::symbol::Symbol;
use result::ObjError;

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
      match env.stream.peek_char() {
         Ok(resp) if is_symbol_start(inp.char_val) => 
      }
      if !is_symbol_start(peek_char!(env, EndOfFile => '0')){
         return PluginResponse::NoResponse
      };

      let mut symbol_acc: String = String::new();

      loop {
         let peeked_char = peek_char!(env, EndOfFile => break);

         if is_symbol_cont(peeked_char) {
            symbol_acc.push(peeked_char);
         } else { 
            break
         }
         
         assert_next_eq!(peeked_char, env);
      }

      if symbol_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         ok_rc!(RESP; Symbol::new(symbol_acc))
      }
   }
}














