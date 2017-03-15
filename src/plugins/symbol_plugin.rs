use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::symbol::Symbol;

#[derive(Debug)]
pub struct SymbolPlugin{}

pub static INSTANCE: SymbolPlugin = SymbolPlugin{};

impl Plugin for SymbolPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let was_first_alphabetical = match env.stream.peek_char() {
         Some(obj) => obj.source_val.is_alphabetic(),
         None => false,
      };
      if !was_first_alphabetical {
         return PluginResponse::NoResponse 
      }

      let mut symbol_acc: String = String::new();

      loop {
         if let Some(peeked_single_character) = env.stream.peek_char() {
            let peeked_next_char = peeked_single_character.source_val;
            if peeked_next_char.is_alphanumeric(){
               symbol_acc.push(peeked_next_char);
            } else {
               break
            }
         } else { break };
         env.stream.next(); // this will only occur if a break isnt called
      }

      if symbol_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         let symbol = Symbol::new(symbol_acc);
         PluginResponse::Response(Ok(Box::new(symbol)))
      }
   }

}














