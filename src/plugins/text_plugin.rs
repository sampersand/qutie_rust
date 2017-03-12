use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use objects::universe::Universe;
use objects::boxed_obj::BoxedObj;
use objects::singlecharacter::SingleCharacter;
use objects::object::ObjectType;
use objects::number::Number;
use objects::text::Text;

#[derive(Debug)]
pub struct TextPlugin{}

pub static INSTANCE: TextPlugin = TextPlugin{};

impl Plugin for TextPlugin {

   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), env.parser);
      match env.stream.next( to_pass ) {
         None => NextObjectResult::NoResponse,
         Some(e) => {
            if match e.obj_type() {
               ObjectType::SingleCharacter(single_char) => {
                  if single_char.source_val.is_whitespace() {
                     false
                  } else {
                     true
                  }
               },
               e @ _ => panic!("Unknown type {:?}", e)
            } {
               env.stream.feed(e, to_pass);
               NextObjectResult::NoResponse         
            } else {
               NextObjectResult::Retry
            }
         }
      }
   }
}














