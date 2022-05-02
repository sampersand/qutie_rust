use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::text::Text;
use objects::obj_rc::ObjRc;
use objects::object::ObjWrapper;
use objects::number::Number;
use objects::boolean::{Boolean, BoolType};

use env::Environment;
use result::{ObjResult, ObjError};

use std::fs::File;
use std::io::Read;
use std::io;

fn import_lib(name: &str, _: &mut Environment) -> ObjResult {
   // we have no inbuilt libs yet
   return Err(ObjError::NoSuchKey(new_obj!(TEXT, name.to_string())));
}
fn import_path(path: &str, _: &mut Environment) -> Result<ObjRc, io::Error>{
   let mut file_text = String::new();
   match File::open(path) {
      Ok(file) => file,
      Err(err) => return Err(err)
   }.read_to_string(&mut file_text)?;
   use parser::Parser;
   let ret = Parser::new().process(file_text.as_str());
   Ok(ret.to_rc())
}
pub fn import_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let name_num  = new_obj!(NUM, 0);
   let do_merge_num = new_obj!(NUM, 1);
   let name_arg  = get_arg!(args, name_num; Stack, panic!("No body block!"));
   let do_merge_arg = get_arg!(args, do_merge_num; Stack, new_obj!(BOOL_STATIC, False));
   let name = to_type!(STRING; name_arg, env);
   let do_merge = to_type!(BOOL; do_merge_arg, env);
   let ret = 
      match import_path(&name, env){
         Ok(obj) => Ok(obj),
         Err(_) => 
            match import_lib(&name, env){
               Ok(obj) => Ok(obj),
               Err(err) => panic!("Cannot open file {:?} for reading: {:?}", name, err)
            }
      };
   if do_merge {
      let ret = ret.expect("err with import");
      let uni = cast_as!(CL; ret, Universe);
      env.universe.merge_vars(uni);
      Ok(ret)
   } else {
      ret
   }
}







