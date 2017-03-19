use objects::object::{Object, ObjType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;
use objects::boolean::Boolean;
use objects::obj_rc::ObjRc;
use std::rc::Rc;
use result::{ObjResult, ObjError, BoolResult};
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
}

macro_rules! num_oper_func {
   ( $name_l:ident, $name_r:ident, $oper:tt ) => {
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => {
               if let ObjType::Number(num_obj) = obj.obj_type() {
                  Ok(Rc::new(Number::new(self.num_val $oper num_obj.num_val )))
               } else { 
                  panic!("Unknown type!")
               }
            },
            Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
   };
   ( $name_l:ident, $name_r:ident, func=$oper:ident ) => {
      fn $name_l(&self, other: &ObjRc, env: &mut Environment) -> ObjResult {
         match other.qt_to_num(env) {
            Ok(obj) => {
               if let ObjType::Number(num_obj) = obj.obj_type() {
                  Ok(Rc::new(Number::new(self.num_val.$oper(num_obj.num_val))))
               } else { 
                  panic!("Unknown type!")
               }
            },
            Err(ObjError::NotImplemented) => Err(ObjError::NotImplemented),
            Err(err) => panic!("Don't know how to deal with error: {:?}", err)
         }
      }
   }

}

impl Object for Number{
   fn obj_type(&self) -> ObjType { ObjType::Number(self) }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.num_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }


   fn qt_to_num(&self, _: &mut Environment) -> Result<Rc<Number>, ObjError> {
      Ok(Rc::new(Number::new(self.num_val)))
   }

   fn qt_eql_l(&self, other: &ObjRc, _: &mut Environment) -> BoolResult {
      let other = match other.obj_type() {
         ObjType::Number(num) => num.num_val,
         _ => return Err(ObjError::NotImplemented)
      };
      Ok(Rc::new(Boolean::from_bool(self.num_val == other)))
   }
   fn qt_to_bool(&self, _: &mut Environment) -> BoolResult {
      Ok(Rc::new(Boolean::from_bool(self.num_val != 0)))
   }
   num_oper_func!(qt_add_l, qt_add_r, +);
   num_oper_func!(qt_sub_l, qt_sub_r, -);
   num_oper_func!(qt_mul_l, qt_mul_r, *);
   num_oper_func!(qt_div_l, qt_div_r, /);
   num_oper_func!(qt_mod_l, qt_mod_r, %);
   // num_oper_func!(qt_pow_l, qt_pow_r, func=powf);
}


impl Display for Number {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.num_val)
   }
}
impl Debug for Number {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "N({})", self)
   }
}












