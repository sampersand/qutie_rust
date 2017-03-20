use env::Environment;
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
   fn next_object(&self, env: &mut Environment) -> PluginResponse {

      let peeked_char = peek_char!(env);

      if !is_lparen(peeked_char) {
         return PluginResponse::NoResponse
      }


      let mut paren_level = 1;
      let mut uni_acc: String = String::new();
      let l_paren: char = peeked_char;
      while 0 < paren_level  {
         env.stream.next(); // will pop the peeked character that was first paren
                                          /* keep it container, it's an old throwback */
         let peek_char = peek_char!(env, EndOfFile => panic!("Reached EOF whilst reading container: {:?}", uni_acc));
         if is_rparen(peek_char) {
            paren_level -= 1
         } else if is_lparen(peek_char) {
            paren_level += 1
         }
         if paren_level != 0 {
            uni_acc.push(peek_char)
         }
      }
      let r_paren = peek_char!(env);
      env.stream.next(); // pop the end

      ok_rc!(RESP; Universe::new(Some([l_paren, r_paren]),
                                 Some(Universe::parse_str(uni_acc.as_str())),
                                 None,
                                 None))
   }
}














