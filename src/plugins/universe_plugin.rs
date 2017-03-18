use objects::obj_rc::ObjRc;

use parser::Parser;
use objects::universe::Universe;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::object::Object;
use objects::single_character::SingleCharacter;
use std::rc::Rc;
use result::ObjError;

#[derive(Debug)]
pub struct UniversePlugin;

pub static INSTANCE: UniversePlugin = UniversePlugin{};
pub const L_PARENS: [char; 3] = ['[', '{', '('];
pub const R_PARENS: [char; 3] = [']', '}', ')'];

fn is_lparen(inp: char) -> bool {
   for paren in L_PARENS.iter() {
      if *paren == inp {
         return true
      }
   }
   false
}

fn is_rparen(inp: char) -> bool {
   for paren in R_PARENS.iter() {
      if *paren == inp {
         return true
      }
   }
   false
}

impl Plugin for UniversePlugin {
   fn next_object(&self,
                  stream: &mut Universe, // stream
                  enviro: &mut Universe, // enviro
                  parser: &Parser,       // parser
                 ) -> PluginResponse {
      let peeked_char = match stream.peek_char() {
         Ok(peeked_struct) => peeked_struct.char_val,
         Err(ObjError::EndOfFile) => return PluginResponse::NoResponse,
         Err(err) => panic!("Unknown error: {:?}", err)
      };

      if !is_lparen(peeked_char) {
         return PluginResponse::NoResponse
      }


      let mut paren_level = 1;
      let mut uni_acc: String = String::new();
      let l_paren: char = peeked_char;
      loop {
         stream.next(); // will pop the peeked character that was first paren
         match stream.peek_char() {
            Ok(peeked_struct) => {
               let peeked_char = peeked_struct.char_val;
               if is_rparen(peeked_char) {
                  break
               } else {
                  uni_acc.push(peeked_char);
               }
            },
            Err(ObjError::EndOfFile) => panic!("Reached EOF whilst container: {:?}", uni_acc),
            Err(_) => panic!("Howto deal with non-eof errors")
         }
      }
      let r_paren = stream.peek_char().unwrap().char_val;
      stream.next(); // pop the end

      let uni = Universe::new(Some([l_paren, r_paren]),
                              Some(Universe::parse_str(uni_acc.as_str())),
                              None,
                              None);
      PluginResponse::Response(Ok(Rc::new(uni)))
   }
}














