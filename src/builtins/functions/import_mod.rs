use std::rc::Rc;
use objects::universe::{Universe, AccessType};
use objects::symbol::Symbol;
use objects::text::Text;
use objects::number::Number;
use objects::boolean;
use objects::object::Object;

use env::Environment;
use result::{ObjResult, ObjError};

use std::fs::File;
use std::io::Read;

fn import_path(path: String, env: &mut Environment) -> ObjResult{
   let mut file_text = String::new();
   let path_clone = path.clone();
   match File::open(path) {
      Ok(file) => file,
      Err(err) => panic!("Cannot open file {:?} for reading: {}", path_clone, err)
   }.read_to_string(&mut file_text);

   use parser::Parser;
   let ret = Parser::new().process(file_text.as_str());
   Ok(rc!(ret))
}
pub fn import_fn(args: Rc<Universe>, env: &mut Environment) -> ObjResult {
   let import_name_num  = new_obj!(NUM, 0);
   let import_name_arg = get_arg!(args, import_name_num; Stack, panic!("No body block!"));
   let import_name = to_type!(STRING; import_name_arg, env);
   import_path(import_name, env)
}