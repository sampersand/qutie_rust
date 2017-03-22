use objects::object::{Object, ObjType};
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
   pub fn to_string(&self) -> String {
      self.num_val.to_string()
   }
}

macro_rules! num_oper_func {
   ( $name_l:ident, $name_r:ident, $oper:tt ) => {
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => {
               if let ObjType::Number(num_obj) = obj.obj_type() {
                  ok_rc!(Number::new(self.num_val $oper num_obj.num_val ))
               } else { 
                  panic!("Unknown type!")
               }
            },
            Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
   };
   (BOOL; $name_l:ident, $name_r:ident, $oper:tt ) => {
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => {
               if let ObjType::Number(num_obj) = obj.obj_type() {
                  ok_rc!(Boolean::from_bool(self.num_val $oper num_obj.num_val ))
               } else { 
                  panic!("Unknown type!")
               }
            },
            Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
   };

   // ( $name_l:ident, $name_r:ident, func=$oper:ident ) => {
   //    fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
   //       match other.qt_to_num(env) {
   //          Ok(obj) => {
   //             if let ObjType::Number(num_obj) = obj.obj_type() {
   //                ok_rc!(Number::new(self.num_val.$oper(num_obj.num_val)))
   //             } else { 
   //                panic!("Unknown type!")
   //             }
   //          },
   //          Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
   //          Err(err) => panic!("Don't know how to deal with error: {:?}", err)
   //       }
   //    }
   // }

}

impl Object for Number{
   impl_defaults!(OBJECT; Number);


   fn qt_to_num(&self, _: &mut Environment) -> Result<Rc<Number>, ObjError> {
      ok_rc!(Number::new(self.num_val))
   }

   obj_functions!(QT_TO_BOOL; (|me: &Number| me.num_val != 0));
   obj_functions!(QT_TO_TEXT);
   // obj_functions!(QT_EQL; Number, num_val);

   num_oper_func!(qt_add_l, qt_add_r, +);
   num_oper_func!(qt_sub_l, qt_sub_r, -);
   num_oper_func!(qt_mul_l, qt_mul_r, *);
   num_oper_func!(qt_div_l, qt_div_r, /);
   num_oper_func!(qt_mod_l, qt_mod_r, %);

   num_oper_func!(BOOL; qt_lth_l, qt_lth_r, <);
   num_oper_func!(BOOL; qt_gth_l, qt_gth_r, >);
   num_oper_func!(BOOL; qt_leq_l, qt_leq_r, <=);
   num_oper_func!(BOOL; qt_geq_l, qt_geq_r, >=);
   num_oper_func!(BOOL; qt_eql_l, qt_eql_r, ==);
   num_oper_func!(BOOL; qt_neq_l, qt_neq_r, !=);
}


impl_defaults!(DISPLAY_DEBUG; Number, 'N');

