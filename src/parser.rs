use globals;

use objects::obj_rc::ObjRc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use objects::object::Object;

use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::universe;
use result::{ObjResult, ObjError};

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::default_plugin;
use plugins::pre_command_plugin;

use env::Environment;

pub type PluginsVec = Vec<&'static Plugin>;

#[derive(Debug, Clone)]
pub struct Parser {
   plugins: RefCell<PluginsVec>,
}

#[derive(Debug)]
pub struct TokenPair(pub ObjResult, pub &'static Plugin);

impl Parser {
	pub fn new() -> Parser {
      let plugins = PluginsVec::new();

		let mut res = Parser {
         plugins: RefCell::new(plugins),
      };
      if res.plugins.borrow().len() == 0 {
         res.add_plugin(default_plugin::INSTANCE);
         res.add_plugin(pre_command_plugin::INSTANCE);
      }
      res
	}

   pub fn add_plugin(&self, plugin: &'static Plugin) {
      self.insert_plugin(0, plugin)
   }
   pub fn insert_plugin(&self, loc: usize, plugin: &'static Plugin) {
      self.plugins.borrow_mut().insert(loc, plugin);
   }

   pub fn del_plugin(&self, plugin: &'static Plugin) -> usize {
      let plugin = plugin as *const Plugin;
      let len = self.plugins.borrow().len();

      let mut pos = len;
      for (i, pl) in self.plugins.borrow().iter().enumerate() {
         if *pl as *const Plugin == plugin {
            pos = i; break
         }
      }
      if pos == len {
         panic!("Plugin not added: {:?}", plugin);
      }
      self.plugins.borrow_mut().remove(pos);
      pos
   }

   pub fn has_plugin(&self, plugin: &'static Plugin) -> bool {
      for pl in self.plugins.borrow().clone() {
         if pl as *const Plugin == plugin as *const Plugin {
            return true;
         }
      }
      false
   }

   pub fn process(&mut self, input: &str) -> Universe {

      let mut universe = Universe::new(Some(['<', '>']), None, None, None);
      {
         let mut stream = Universe::new(Some(['<', '>']), Some(Universe::parse_str(input)), None, None);
         let parser = rc!(self);
         let mut env = Environment::new(&mut stream, &mut universe, parser);
         env.parser.clone().parse(&mut env);
      }
      universe
   }

   pub fn parse(&self, env: &mut Environment) {
      // let old_global_env = globals::GLOBAL_ENV;
      // globals::GLOBAL_ENV = env;
      while !env.stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(env);
         match token {
            Ok(obj) => plugin.handle(obj, env),
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Uncaught error: {:?}", err),
         }
      }
      // globals::GLOBAL_ENV = old_global_env;
   }

   pub fn next_object(&self, env: &mut Environment) -> TokenPair {
      let plugins = self.plugins.clone();
      for pl in &*plugins.borrow() {
         match pl.next_object(env) {
            PluginResponse::NoResponse => {},
            PluginResponse::Retry => return self.next_object(env),
            PluginResponse::Response(obj) => return TokenPair(obj, *pl),
         }
      }
      TokenPair(Err(ObjError::EndOfFile), default_plugin::INSTANCE)
   }

}





















