use plugins::plugin::Plugin;
use environment::Environment;
use plugins::plugin::PluginResponse;
use objects::symbol::Symbol;
use result::ObjErr;

#[derive(Debug)]
pub struct SymbolPlugin;

pub static INSTANCE: SymbolPlugin = SymbolPlugin{};

impl Plugin for SymbolPlugin {

   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      let was_first_alphabetical = match env.stream.peek_char() {
         Ok(obj) => obj.source_val.is_alphabetic(),
         Err(ObjErr::EndOfFile) => false,
         Err(err) => panic!("Don't know how to deal with error: {:?}", err)
      };
      if !was_first_alphabetical {
         return PluginResponse::NoResponse 
      }

      let mut symbol_acc: String = String::new();

      loop {
         match env.stream.peek_char() {
            Ok(peeked_struct) => {
               let peeked_char = peeked_struct.source_val;
               if peeked_char.is_alphanumeric(){
                  symbol_acc.push(peeked_char);
               } else {
                  break
               }
            },
            Err(ObjErr::EndOfFile) => break,
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
         let _next_char = env.stream.next(); // this will only occur if a break isnt called
      }

      if symbol_acc.is_empty() {
         PluginResponse::NoResponse
      } else {
         let symbol = Symbol::new(symbol_acc);
         PluginResponse::Response(Ok(Box::new(symbol)))
      }
   }

}














