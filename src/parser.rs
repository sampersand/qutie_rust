use globals;

use objects::obj_rc::ObjRc;
use std::collections::HashMap;
use std::cell::RefCell;

use objects::object::Object;

use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::universe;
use result::{ObjResult, ObjError};

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::default_plugin::DefaultPlugin;
use plugins::default_plugin;

use env::Environment;

pub type BuiltinsMap = universe::GlobalsType;
pub type PluginsVec = Vec<&'static Plugin>;

#[derive(Debug)]
pub struct Parser {
   plugins: RefCell<PluginsVec>,
   builtins: RefCell<BuiltinsMap>,
}
#[derive(Debug)]
pub struct TokenPair(pub ObjResult, pub &'static Plugin);

impl Parser {
	pub fn new(plugins: RefCell<PluginsVec>, builtins: RefCell<BuiltinsMap>) -> Parser {
		let mut res = Parser{ plugins: plugins, builtins: builtins};
      res.add_plugin(default_plugin::INSTANCE);
      res
	}

   pub fn add_plugin(&mut self, plugin: &'static Plugin) {
      self.plugins.borrow_mut().insert(0, plugin);
   }

   pub fn add_builtins(&mut self, builtins: BuiltinsMap) {
      self.builtins.borrow_mut().extend(builtins);
   }

   pub fn fork(&self) -> Parser {
      panic!()
   }

   pub fn process(&mut self, input: &str) -> Universe {
      let mut stream = Universe::new(Some(['<', '>']), Some(Universe::parse_str(input)), None, None);
      let mut universe = Universe::new(Some(['<', '>']), None, None, None);
      universe.globals.extend(self.builtins.clone().into_inner());
      {
         let forked = self.fork();
         let mut env = Environment::new(&mut stream, &mut universe, self);
         forked.parse(&mut env);
      }
      universe
   }

   pub fn parse(&self, env: &mut Environment) {
      // let old_global_env = globals::GLOBAL_ENV;
      // globals::GLOBAL_ENV = env;
      while !env.stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(env);
         match token {
            Ok(boxed_obj) => (*plugin).handle(boxed_obj, env),
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Uncaught error: {:?}", err),
         }
      }
      // globals::GLOBAL_ENV = old_global_env;
   }

   pub fn next_object(&self, env: &mut Environment) -> TokenPair {
      for pl in &*self.plugins.borrow() {
         match pl.next_object(env) {
            PluginResponse::NoResponse => {},
            PluginResponse::Retry => return self.next_object(env),
            PluginResponse::Response(obj) => return TokenPair(obj, *pl),
         }
      }
      TokenPair(Err(ObjError::EndOfFile), default_plugin::INSTANCE)
   }

}





















