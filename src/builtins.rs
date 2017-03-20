use objects::object::Object;
use objects::universe::GlobalsType;
use objects::symbol::Symbol;
use objects::obj_rc::ObjRcWrapper;
use objects::boolean;
use std::rc::Rc;
macro_rules! map {
   { $($key:expr => $value:expr),+ } => {
      {
         let mut m = GlobalsType::new();
         $(
            m.insert(ObjRcWrapper(Rc::new(Symbol::from($key))), $value);
         )+
         m
      }
   }
}

pub fn builtins() -> GlobalsType {
   let null = Rc::new(boolean::NULL);
   let builtins: GlobalsType = map! {
      "true" => Rc::new(boolean::TRUE),
      "false" => Rc::new(boolean::FALSE),
      "null" => null.clone(),
      "nil" => null.clone(),
      "none" => null.clone()
   };
   builtins
}