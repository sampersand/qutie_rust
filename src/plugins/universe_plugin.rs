use env::Environment;
use objects::obj_rc::ObjRc;

use parser::Parser;
use objects::universe::Universe;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};
use objects::object::Object;
use objects::single_character::SingleCharacter;
use std::rc::Rc;
use result::ObjError;

#[derive(Debug)]
pub struct UniversePlugin;

pub static INSTANCE: &'static UniversePlugin = &UniversePlugin{};
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

      let l_paren = peek!(env, is_lparen, return NoResponse);

      let mut paren_level = 1;
      let mut uni_acc: String = String::new();


      while 0 < paren_level  {
         env.stream.next(); // will pop the peeked character that was first paren
                                          /* keep it container, it's an old throwback */
         let peek = peek!(env, panic!("Reached EOF whilst reading container: {:?}", uni_acc));
         if is_rparen(peek) {
            paren_level -= 1
         } else if is_lparen(peek) {
            paren_level += 1
         }
         if paren_level != 0 {
            uni_acc.push(peek)
         }
      }

      let r_paren = peek!(env);
      assert_next_eq!(r_paren, env);

      ok_rc!(RESP; Universe::new(Some([l_paren, r_paren]),
                                 Some(Universe::parse_str(uni_acc.as_str())),
                                 None,
                                 None))
   }
}














