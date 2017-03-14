use objects::object::{Object, ObjectType, FunctionResponse, FunctionError};
use objects::boxed_obj::BoxedObj;
use objects::universe::Universe;
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;
use environment::Environment;


macro_rules! oper_func {
    ( $name:ident, $name_l:ident, $name_r:ident ) => {

         fn $name(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> FunctionResponse {
            let l = l.unwrap();
            let r = r.unwrap();
            match l.$name_l(&r) {
               e @ Ok(_) => e,
               Err(err) => match err {
                  e @ FunctionError::VoidResponse => Err(e),
                  FunctionError::NoResponse => r.$name_r(&l)
               }
            }
         }
    };
}

macro_rules! new_oper {
   ($symbol:expr, $priority:expr, $func:ident, $oper_type:expr) => {
      Operator{
         symbol: $symbol,
         priority: $priority,
         has_lhs: true,
         has_rhs: true,
         oper_type: $oper_type,
         func: $func
      };
   };
   ($symbol:expr, $priority:expr, $func:ident, $has_lhs:expr, $has_rhs:expr, $oper_type:expr) => {
      Operator{
         symbol: $symbol,
         priority: $priority,
         has_lhs: $has_lhs,
         has_rhs: $has_rhs,
         oper_type: $oper_type,
         func: $func
      };
   }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperatorType {
   Add, Sub, Mul, Div, Mod, Pow,
   Eql, Neq, Lth, Gth, Leq, Geq,
   Cmp, Rgx,
   Endl, Sep, 
}

pub struct Operator{
   pub symbol: &'static str,
   pub priority: u32,
   pub has_lhs: bool,
   pub has_rhs: bool,
   pub oper_type: OperatorType,
   pub func: fn(Option<BoxedObj>, Option<BoxedObj>, &mut Environment) -> FunctionResponse,
}



fn endl_fnc(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> FunctionResponse { Err(FunctionError::VoidResponse) }
fn sep_fnc(l: Option<BoxedObj>, r: Option<BoxedObj>, env: &mut Environment) -> FunctionResponse { Ok(l.unwrap()) }

oper_func!(qt_add, qt_add_l, qt_add_r);
oper_func!(qt_sub, qt_sub_l, qt_sub_r);
oper_func!(qt_mul, qt_mul_l, qt_mul_r);
oper_func!(qt_div, qt_div_l, qt_div_r);
oper_func!(qt_mod, qt_mod_l, qt_mod_r);
oper_func!(qt_pow, qt_pow_l, qt_pow_r);

oper_func!(qt_eql, qt_eql_l, qt_eql_r);
oper_func!(qt_neq, qt_neq_l, qt_neq_r);
oper_func!(qt_gth, qt_gth_l, qt_gth_r);
oper_func!(qt_lth, qt_lth_l, qt_lth_r);
oper_func!(qt_geq, qt_geq_l, qt_geq_r);
oper_func!(qt_leq, qt_leq_l, qt_leq_r);

oper_func!(qt_cmp, qt_cmp_l, qt_cmp_r);
oper_func!(qt_rgx, qt_rgx_l, qt_rgx_r);


use std::collections::HashMap;
macro_rules! map( /* from http://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal */
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


lazy_static! {
    pub static ref OPERATORS: HashMap<OperatorType, Operator> = map!{
      OperatorType::Add => new_oper!("+",  12, qt_add, OperatorType::Add),
      OperatorType::Sub => new_oper!("-",  12, qt_sub, OperatorType::Sub),
      OperatorType::Mul => new_oper!("*",  11, qt_mul, OperatorType::Mul),
      OperatorType::Div => new_oper!("/",  11, qt_div, OperatorType::Div),
      OperatorType::Mod => new_oper!("%",  11, qt_mod, OperatorType::Mod),
      OperatorType::Pow => new_oper!("**", 10, qt_pow, OperatorType::Pow),
      OperatorType::Sep => new_oper!(",",  40, sep_fnc, true, false, OperatorType::Sep),
      OperatorType::Endl => new_oper!(";", 40, endl_fnc, true, false, OperatorType::Endl)
    };
}


impl Clone for Operator{
   fn clone(&self) -> Operator {
      Operator{symbol: self.symbol.clone(),
               priority: self.priority.clone(),
               has_lhs: self.has_lhs.clone(),
               has_rhs: self.has_rhs.clone(),
               oper_type: self.oper_type.clone(),
               func: self.func}
   }
}

impl Object for Operator {
   fn obj_type(&self) -> ObjectType { ObjectType::Operator(self) }
   fn source(&self) -> Vec<SingleCharacter>{
      let mut ret = vec![];
      for chr in self.symbol.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }
}

impl Display for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.symbol)
   }
}
impl Debug for Operator{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "O({})", self)
   }
}