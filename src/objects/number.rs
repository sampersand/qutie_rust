
use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use objects::boolean::Boolean;
use objects::text::Text;
use objects::obj_rc::ObjRc;
use std::rc::Rc;
use result::{ObjResult, ObjError};
use objects::universe::Universe;
use parser::Parser;
use env::Environment;

pub type NumberType = i32;

pub struct Number {
   pub num_val: NumberType
}

impl Number {
   pub fn new(inp: NumberType) -> Number {
      Number{num_val: inp}
   }
   pub fn to_rc(self) -> Rc<Number> {
      Rc::new(self)
   }

   pub fn to_string(&self) -> String {
      self.num_val.to_string()
   }
}

macro_rules! num_oper_func {
   ( $ret_type:ident; $name_l:ident, $name_r:ident, $oper:tt ) => {
      fn $name_l(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => Ok(new_obj!($ret_type, self.num_val $oper obj.num_val )),
            Err(ObjError::NotImplemented) => self.$name_r(other, env),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
      fn $name_r(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => Ok(new_obj!($ret_type, obj.num_val $oper self.num_val )),
            Err(ObjError::NotImplemented) => self.$name_r(other, env),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
   };
}

impl Object for Number{
   impl_defaults!(OBJECT; Number);
   obj_functions!(QT_TO_TEXT);

   fn qt_to_num(&self, _: &mut Environment) -> Result<Rc<Number>, ObjError> {
      Ok(new_obj!(NUM, self.num_val))
   }
   fn qt_to_bool(&self, _: &mut Environment) -> Result<Rc<Boolean>, ObjError> {
      Ok(new_obj!(BOOL, self.num_val != 0))
   }

   num_oper_func!(NUM; qt_add_l, qt_add_r, +);
   num_oper_func!(NUM; qt_sub_l, qt_sub_r, -);
   num_oper_func!(NUM; qt_mul_l, qt_mul_r, *);
   num_oper_func!(NUM; qt_div_l, qt_div_r, /);
   num_oper_func!(NUM; qt_mod_l, qt_mod_r, %);

   num_oper_func!(BOOL; qt_lth_l, qt_lth_r, <); 
   num_oper_func!(BOOL; qt_gth_l, qt_gth_r, >);
   num_oper_func!(BOOL; qt_leq_l, qt_leq_r, <=);
   num_oper_func!(BOOL; qt_geq_l, qt_geq_r, >=);
   num_oper_func!(BOOL; qt_eql_l, qt_eql_r, ==);
   num_oper_func!(BOOL; qt_neq_l, qt_neq_r, !=);
}


impl_defaults!(DISPLAY_DEBUG; Number, 'N');

