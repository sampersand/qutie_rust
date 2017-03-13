use std::fmt::{Debug, Display};
use objects::single_character;
use objects::operator;
use objects::boxed_obj::BoxedObj;
use objects::number;

#[derive(Debug)]
pub enum QTFunctionResponse{
   Unimplemented,
   Response(BoxedObj)
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


   fn qt_add_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_add_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

   fn qt_sub_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_sub_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

   fn qt_mul_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_mul_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

   fn qt_div_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_div_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

   fn qt_mod_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_mod_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

   fn qt_pow_l(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }
   fn qt_pow_r(&self, other: BoxedObj) -> QTFunctionResponse { unimplemented!() }

}