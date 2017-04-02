use parser::Parser;
use env::Environment;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::plugin::PluginResponse::{NoResponse, Response};

use objects::obj_rc::ObjRc;
use objects::universe::Universe;


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

      let l_paren = match env.stream.peek() {
                       Some(ref mut c) if is_lparen(c.chr) => c.take(),
                       _ => return NoResponse
                    };

      let mut paren_level = 1;
      let mut uni_acc: String = String::new();


      while 0 < paren_level  {
         let mut peeked = env.stream.peek().expect("Reached EOF whilst looking for end of container");
         if is_rparen(peeked.chr) {
            paren_level -= 1
         } else if is_lparen(peeked.chr) {
            paren_level += 1
         }
         if paren_level != 0 {
            uni_acc.push(peeked.take())
         }
      }
      let r_paren = match env.stream.peek() {
         Some(ref mut c) => c.take(),
         None => panic!("Reached EOF whilst looking for end of container")
      };

      Response(ok_rc!(Universe::new(Some([l_paren, r_paren]),
                                    Some(Universe::parse_str(uni_acc.as_str())),
                                    None,
                                    None)))
   }
}














