use std::fmt::{Debug, Display};
use objects::single_character;
use objects::operator;
use objects::boxed_obj::BoxedObj;
use objects::number;
use objects::text;

#[derive(Debug)]
pub enum FunctionError{}

#[derive(Debug)]
pub enum Response {
   Return(BoxedObj),
   NoResponse,
   VoidReturn,
   NotImplemented,
   Error(FunctionError),
   EndOfFile,
}

#[derive(Debug)]
pub enum ObjectType<'a> {
   Universe,
   Number(&'a number::Number),
   SingleCharacter(&'a single_character::SingleCharacter),
   Symbol,
   Text,
   Boolean,
   Operator(&'a operator::Operator),
   Null,
}

pub trait Object : Debug + Display {
   fn obj_type(&self) -> ObjectType;
   fn source(&self) -> Vec<single_character::SingleCharacter>;

   fn qt_to_num(&self) -> Response { Response::NoResponse }
   fn qt_to_text(&self) -> Response { Response::NoResponse }

   fn qt_add_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_sub_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_mul_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_div_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_mod_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_pow_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_add_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_sub_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_mul_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_div_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_mod_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_pow_r(&self, other: &BoxedObj) -> Response { unimplemented!() }

   fn qt_eql_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_neq_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_gth_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_lth_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_leq_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_geq_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_eql_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_neq_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_gth_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_lth_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_leq_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_geq_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
   
   fn qt_cmp_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
   fn qt_rgx_l(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_cmp_r(&self, other: &BoxedObj) -> Response { unimplemented!() }
      fn qt_rgx_r(&self, other: &BoxedObj) -> Response { unimplemented!() }

}







