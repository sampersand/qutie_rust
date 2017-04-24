use env::Environment;

use objects::object::ObjType;
use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::universe_plugin;
use objects::symbol::Symbol;
use objects::operator::{call_fn, exec_fn};

#[derive(Debug)]
pub struct AutoFunctionCall;

pub static INSTANCE: &'static AutoFunctionCall = &AutoFunctionCall{};

impl Plugin for AutoFunctionCall {
   #[allow(unused_must_use)]
   fn next_object(&self, env: &mut Environment) -> PluginResponse {
      // println!("stream: {:?}", env.stream);
      match env.universe.stack.last() {
         Some(obj)
            if (//obj.is_a(ObjType::Universe) || 
                  obj.is_a(ObjType::BuiltinFunction) ||
                  obj.is_a(ObjType::BuiltinMethod) ||
                  obj.is_a(ObjType::UserFunction) ||
                  obj.is_a(ObjType::UserClass)) => {},
         _ => return PluginResponse::NoResponse
      }
      // println!("found_autocall: {:?}", env.universe.stack.last().unwrap());
      // println!("env: {:?}", env);
      // println!("next_uni: {:?}", universe_plugin::INSTANCE.next_object(env));
      let args =
         match universe_plugin::INSTANCE.next_object(env) {
           PluginResponse::Response(obj) => obj.expect("error with auto function calling"),
           PluginResponse::NoResponse => return PluginResponse::NoResponse,
           PluginResponse::Retry => unreachable!("Why is retry being returned from universe?")
         };

      let func = env.universe.stack.pop().expect("No function on the stack .-.");

      let args = exec_fn(Some(args), None, env).expect("exec_fn returned error");
      if func.is_a(ObjType::UserClass) {
         let response = call_fn(Some(func), Some(env.universe.to_globals().to_rc()), env);
         PluginResponse::Response(
            match response {
               Ok(obj) => {
                  if let Ok(__init) = get_method!(CL; obj, "__init", env)  {
                     call_fn(Some(__init), Some(args), env);
                  }
                  Ok(obj)
               }
               Err(err) => Err(err)
            })
      } else {
         let response = call_fn(Some(func), Some(args), env);
         PluginResponse::Response(response)
      }

   }
}














