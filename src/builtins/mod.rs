use objects::universe::GlobalsType;

pub mod constants;
pub mod functions;

pub type BuiltinsType = GlobalsType;

pub fn builtins() -> BuiltinsType {
   let mut ret = BuiltinsType::new();
   ret.extend(functions::functions());
   ret.extend(constants::constants());
   ret
}
