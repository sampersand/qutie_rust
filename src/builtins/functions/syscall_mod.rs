use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::{Text, Quote};
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

use std::fs::File;
use std::io::Read;
pub fn syscall_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let cmd_pos  = rc_obj!(NUM; 0);
   let cmd_obj = get_arg!(args, cmd_pos; Stack, panic!("No body block!"));
   let mut args_obj_ary = args.stack.clone();
   args_obj_ary.remove(0);

   let cmd = to_type!(STRING; cmd_obj, env);
   let mut args_str_ary = vec![];
   for arg_obj in args_obj_ary {
      args_str_ary.push(to_type!(STRING; arg_obj, env));
   }
   
   use std::process::Command;
   let output = Command::new(cmd).args(&args_str_ary).output().expect("failed to execute cmd").stdout;
   let result = if output.is_empty() {
                  String::new()
                } else {
                  String::from_utf8_lossy(&output[0..output.len() - 1]).into_owned()
                };
   ok_rc!(Text::new(result, None))
}











