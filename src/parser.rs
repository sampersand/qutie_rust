use environment::{Environment, OwnedEnvironment};
use std::collections::HashMap;

use objects::object::Object;
use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::boxed_obj::BoxedObj;
use objects::universe;

use plugins::plugin::Plugin;
use plugins::NextObjectResult;
use plugins::default_plugin::DefaultPlugin;
use plugins::default_plugin;

type BuiltinsMap = universe::LocalsType;
type PluginsVec = Vec<&'static Plugin>;

#[derive(Debug)]
pub struct Parser {
   plugins: PluginsVec,
   builtins: BuiltinsMap,
}

#[derive(Debug)]
pub struct TokenPair(pub BoxedObj, pub &'static Plugin);

impl Parser {
	pub fn new() -> Parser {
		let mut res = Parser{ plugins: PluginsVec::new(), builtins: BuiltinsMap::new() };
      res.add_plugin(&default_plugin::INSTANCE);
      res
	}

   pub fn add_plugin(&mut self, plugin: &'static Plugin) {
      self.plugins.insert(0, plugin);
   }

   pub fn add_builtins(&mut self, builtins: BuiltinsMap) {
      unimplemented!();
   }

   pub fn process(&self, input: &str) -> OwnedEnvironment { // there are more thigns here that we dont need rn
      let mut stream = Universe::new();
      let mut universe = Universe::new();

      let mut env = OwnedEnvironment::new(stream, universe, self);
      for chr in input.chars() {
         env.stream.push( Box::new(SingleCharacter::new(chr)));
      }
      self.parse(&mut env.to_unowned());
      env
   }
   pub fn parse(&self, env: &mut Environment) {
      let mut i = 0;
      while !env.stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(env);
         (*plugin).handle(token, env);
         i += 1;
         if i >= 20{
            panic!("{:?} >= 20: {:?}", i, env );
         }
      }
   }
   pub fn next_object(&self, env: &mut Environment) -> TokenPair {
      for pl in &(self.plugins) {
         match pl.next_object(env) {
            NextObjectResult::NoResponse => {},
            NextObjectResult::Retry => { 
               return self.next_object(env);
            },
            NextObjectResult::Response(response) => {
               return TokenPair(response, *pl);
            }
         }
      }
      panic!("No applicable plugin found for stream: {:?}", env.stream);
   }
}





















