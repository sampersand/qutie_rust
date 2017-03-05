use objects::Object;
use environment::Environment;
use std;
pub trait Plugin : std::fmt::Debug {
   fn next_object(&self, arg: Environment) -> Object;
   fn handle(&self, arg: Environment) -> ();
}