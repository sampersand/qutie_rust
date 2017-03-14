use objects::boxed_obj::BoxedObj;
use environment::Environment;
use std;

#[derive(Debug)]
pub enum PluginResponse {
   NoResponse,
   Retry,
   Response(BoxedObj)
}
pub mod plugin;
pub mod default_plugin;
pub mod number_plugin;
pub mod whitespace_plugin;
pub mod text_plugin;
pub mod symbol_plugin;
// pub mod operator_plugin;
