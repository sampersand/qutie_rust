use env::Environment;

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use objects::universe::{Universe, ParenType};

use objects::object::{ObjType, ObjWrapper};
use objects::boolean::{Boolean, BoolType};
use objects::obj_rc::ObjRc;
use std::rc::Rc;

#[derive(Debug)]
pub struct UniversePlugin;

pub static INSTANCE: &'static UniversePlugin = &UniversePlugin{};

impl Plugin for UniversePlugin {
   fn next_object(&self, env: &mut Environment) -> PluginResponse {

      let l_paren =
         match env.stream.peek() {
            Some(ref mut c) => 
               if let Some(paren) = ParenType::from_char(c.chr, false) {
                  let __tmp_c = c.take();
                  assert_debug!(eq; __tmp_c, paren.to_char(false));
                  paren
               } else {
                  return PluginResponse::NoResponse
               },
            _ => return PluginResponse::NoResponse
         };

      let mut paren_level = 1;
      let mut uni_acc: String = String::new();


      while 0 < paren_level  {
         let mut peeked = env.stream.peek().expect("Reached EOF whilst looking for end of container");
         if let Some(paren) = ParenType::from_char(peeked.chr, true) {
            if paren != ParenType::Angled {
               paren_level -= 1
            }
         } else if let Some(paren) = ParenType::from_char(peeked.chr, false){
            if paren != ParenType::Angled {
               paren_level += 1
            }
         }
         if paren_level != 0 {
            uni_acc.push(peeked.take())
         }
      }
      let r_paren = ParenType::from_char(env.stream.
                                             peek().
                                             expect("Reached EOF whilst looking for end of container").
                                             take(),
                                         true).expect("Error! couldn't unwrap paren");

      let parens = Some([l_paren, r_paren]);
      let stack = Some(Universe::parse_str(uni_acc.as_str()));
      let locals = None;
      let globals = None;
      let uni = Universe::new(parens, stack, locals, globals).to_rc();
      resp_ok!(uni)
   }
   fn handle(&self, token: ObjRc, env: &mut Environment) {

      assert_debug!(is_a; token, Universe);
      let token_uni = cast_as!(token, Universe);
      let to_push = 
         match token_uni.parens[0] {
            ParenType::Curly => token_uni,
            ParenType::Round | ParenType::Square => 
               {
                  let uni = token_uni.exec(env).expect("Error with handling universe");
                  assert_debug!(is_a; uni, Universe);
                  if token_uni.parens[0] == ParenType::Round {
                     match cast_as!(uni, Universe).stack.last() {
                        Some(uni) => uni.clone(),
                        None => new_obj!(BOOL_STATIC, Null)
                     }
                  } else {
                     assert_debug!(eq; token_uni.parens[0], ParenType::Square);
                     uni
                  }
               },
            ParenType::Angled => panic!("Shouldn't be handling angled brackets")
         };
      env.universe.push(to_push)
   }
}














