use plugins::plugin::Plugin;
use environment::Environment;
use plugins::NextObjectResult;
use plugins::NextObjectResult::{NoResponse, Response};
use objects::symbol::Symbol;

#[derive(Debug)]
pub struct SymbolPlugin{}

pub static INSTANCE: SymbolPlugin = SymbolPlugin{};

impl Plugin for SymbolPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      if match env.stream.peek_char() { // aka, if the first char isn't valid, no response.
         None => true,
         Some(obj) => !obj.source_val.is_alphabetic() 
      }{ return NoResponse }
      let mut symbol_acc: String = String::new();
      loop {
         if let Some(obj) = env.stream.peek_char() {
            if obj.source_val.is_alphanumeric(){ symbol_acc.push(obj.source_val); }
            else { break }
         } else { break };
         env.stream.next(); // this will only occur if a break isnt called
      }
      if symbol_acc.is_empty() {
         NoResponse
      } else {
         Response(Box::new(Symbol::new(symbol_acc)))
      }
   }
}














