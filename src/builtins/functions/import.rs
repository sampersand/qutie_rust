use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::obj_rc::ObjRc;
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

use std::fs::File;
use std::io::Read;
use std::io;

fn import_lib(name: &str, env: &mut Environment) -> ObjResult {
   // we have no inbuilt libs yet
   return Err(ObjError::NoSuchKey(new_obj!(TEXT, name.to_string())));
}
fn import_path(path: &str, env: &mut Environment) -> Result<ObjRc, io::Error>{
   let mut file_text = String::new();
   match File::open(path) {
      Ok(file) => file,
      Err(err) => return Err(err)
   }.read_to_string(&mut file_text);
   use parser::Parser;
   let ret = Parser::new().process(file_text.as_str());
   Ok(ret.to_rc())
}
pub fn import_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let import_name_num  = new_obj!(NUM, 0);
   let import_name_arg = get_arg!(args, import_name_num; Stack, panic!("No body block!"));
   let import_name = to_type!(STRING; import_name_arg, env);
   match import_path(&import_name, env){
      Ok(obj) => Ok(obj),
      Err(_) => 
         match import_lib(&import_name, env){
            Ok(obj) => Ok(obj),
            Err(err) => panic!("Cannot open file {:?} for reading: {:?}", import_name, err)
         }
   }
}







