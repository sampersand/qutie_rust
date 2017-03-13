use objects::object::{Object, ObjectType, QTFunctionResponse};
use objects::boxed_obj::BoxedObj;
use objects::universe::Universe;
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;
use environment::Environment;

pub enum OperatorFunctionResponse{
   DontAdd,
   DoAdd(QTFunctionResponse),

}
pub struct Operator2{
   symbol: &'static str,
   priority: u32
}

#[derive(Clone)]
pub enum Operator {
   AssignmentColon,
   AssignmentLeftArrow,
   AssignmentRightArrow,
   Compare,
   Pow,
   EqualTo,
   NotEqualTo,
   LessOrEqual,
   GreaterOrEqual,
   // DotAssign,
   LogicalAnd,
   LogicalOr,
   // DotStack,
   // DotLocals,
   // DotGlobals,
   Multiplication,
   Division,
   Modulus,
   Addition,
   Subtraction,
   LessThan,
   GreaterThan,
   Assignment,
   Call,
   Accessor,
   Endline,
   Seperator,
   Retrieve,
   Evaluate,
}
   
impl Operator {
   pub fn symbol(&self) -> &str {
      match *self {
         Operator::AssignmentColon => ":",
         Operator::AssignmentLeftArrow => "<-",
         Operator::AssignmentRightArrow => "->",
         Operator::Compare => "<=>",
         Operator::Pow => "**",
         Operator::EqualTo => "==",
         Operator::NotEqualTo => "!=",
         Operator::LessOrEqual => "<=",
         Operator::GreaterOrEqual => ">=",
         // Operator::DotAssign => ".=",
         Operator::LogicalAnd => "&&",
         Operator::LogicalOr => "||",
         // Operator::DotStack => ".S",
         // Operator::DotLocals => ".L",
         // Operator::DotGlobals => ".G",
         Operator::Multiplication => "*",
         Operator::Division => "/",
         Operator::Modulus => "%",
         Operator::Addition => "+",
         Operator::Subtraction => "-",
         Operator::LessThan => "<",
         Operator::GreaterThan => ">",
         Operator::Assignment => "=",
         Operator::Call => "@",
         Operator::Accessor => ".",
         Operator::Endline => ";",
         Operator::Seperator => ",",
         Operator::Retrieve => "?",
         Operator::Evaluate => "!",
      }
   }
   pub fn priority(&self) -> u32 {
      match *self {
         Operator::AssignmentColon => 36,
         Operator::AssignmentLeftArrow => 30,
         Operator::AssignmentRightArrow => 30,
         Operator::Compare => 19,
         Operator::Pow => 10,
         Operator::EqualTo => 20,
         Operator::NotEqualTo => 20,
         Operator::LessOrEqual => 20,
         Operator::GreaterOrEqual => 20,
         // Operator::DotAssign => 6,
         Operator::LogicalAnd => 24,
         Operator::LogicalOr => 25,
         // Operator::DotStack => 5,
         // Operator::DotLocals => 5,
         // Operator::DotGlobals => 5,
         Operator::Multiplication => 11,
         Operator::Division => 11,
         Operator::Modulus => 11,
         Operator::Addition => 12,
         Operator::Subtraction => 12,
         Operator::LessThan => 20,
         Operator::GreaterThan => 20,
         Operator::Assignment => 20,
         Operator::Call => 7,
         Operator::Accessor => 5,
         Operator::Endline => 40,
         Operator::Seperator => 40,
         Operator::Retrieve => 1,
         Operator::Evaluate => 1,
      }
   }
   pub fn lhs_call(&self, lhs: BoxedObj, rhs: BoxedObj, env: &mut Environment) -> OperatorFunctionResponse {
      OperatorFunctionResponse::DoAdd(
         match *self{
            Operator::Addition => {
               lhs.qt_add_l(rhs)
            },
            Operator::Endline => { QTFunctionResponse::Response(lhs) },
            _ => return OperatorFunctionResponse::DontAdd
         }
      )
   }
   pub fn list() -> Vec<Operator>{
      vec![Operator::Addition, Operator::Assignment, Operator::Endline]
   }
   pub fn has_lhs(&self) -> bool{
      match *self{
         _ => true
      }
   }
   pub fn has_rhs(&self) -> bool{
      match *self{
         Operator::Endline | Operator::Seperator |
            Operator::Retrieve | Operator::Evaluate => false,
         _ => true
      }
   }
}



impl Object for Operator {
   fn obj_type(&self) -> ObjectType { ObjectType::Operator(self) }
   fn source(&self) -> Vec<SingleCharacter>{
      let mut ret = vec![];
      for chr in self.symbol().to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
}

impl Display for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.symbol())
   }
}
impl Debug for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "O({})", self)
   }
}