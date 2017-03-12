use std::fmt::Debug;
pub fn log<T: Debug>(msg: &T){
   println!("LOG: {:?}", msg);
}