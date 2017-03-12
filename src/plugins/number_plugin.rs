use plugins::plugin::Plugin;
use environment::Environment;
use plugins::next_object_result::NextObjectResult;
use objects::universe::Universe;
use objects::boxed_obj::BoxedObj;
use objects::singlecharacter::SingleCharacter;
use objects::object::ObjectType;
use objects::number::Number;

#[derive(Debug)]
pub struct NumberPlugin{}

pub static INSTANCE: NumberPlugin = NumberPlugin{};

impl NumberPlugin {
   fn next_base(&self, env: &mut Environment) -> NextObjectResult {
      NextObjectResult::NoResponse
   }
   fn next_float(&self, env: &mut Environment) -> NextObjectResult {
      NextObjectResult::NoResponse
   }
   fn next_int(&self, env: &mut Environment) -> NextObjectResult {
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), env.parser);
      let mut result: String = String::new();
      loop {
         match env.stream.next(to_pass) {
            None => { println!("nothing left!"); break },
            Some(obj) => {
               if /*(*/ match obj.obj_type(){
                  ObjectType::SingleCharacter(single_char) => {
                     if single_char.source_val.is_digit(10){
                        result += single_char.source_val.to_string().as_str();
                        false
                     } else {
                        true
                     }
                  }
                  e @ _ => panic!("Unknown type {:?}", e)
               } /*)*/ /* then */ {
                  env.stream.feed(obj, to_pass);
                  break;
               }
            }, 
         };
      }
      if result.is_empty() {
         NextObjectResult::NoResponse
      } else {
         type NumberReturnType = i32;
         NextObjectResult::Response(
            Box::new( Number::<NumberReturnType>::new(result.parse::<NumberReturnType>().unwrap()) )
         )
      }
   }

}

impl Plugin for NumberPlugin {
   fn next_object(&self, env: &mut Environment) -> NextObjectResult {
      match self.next_base(env) {
         NextObjectResult::NoResponse => match self.next_float(env) {
            NextObjectResult::NoResponse => self.next_int(env),
            e @ _ => e,
         },
         e @ _ => e,
      }
   }
}














