pub mod plugin;
pub mod default_plugin;
pub mod number_plugin;
pub mod whitespace_plugin;
pub mod comment_plugin;
pub mod text_plugin;
pub mod symbol_plugin;
pub mod operator_plugin;
pub mod universe_plugin;
pub mod pre_command_plugin;
use std::collections::HashMap;
use std::rc::Rc;
use objects::symbol::Symbol;
use objects::obj_rc::ObjRcWrapper;

type PluginsType = HashMap<ObjRcWrapper, &'static plugin::Plugin>;

pub fn plugins() -> PluginsType {
   map! { TYPE; PluginsType,
      "Number" => number_plugin::INSTANCE,
      "Symbol" => symbol_plugin::INSTANCE,
      "Text" => text_plugin::INSTANCE,
      "Whitespace" => whitespace_plugin::INSTANCE,
      "Universe" => universe_plugin::INSTANCE,
      // "Default" => default_plugin::INSTANCE,
      "Operator" => operator_plugin::INSTANCE,
      "Comment" => comment_plugin::INSTANCE
   }
}