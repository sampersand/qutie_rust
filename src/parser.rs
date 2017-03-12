use environment::Environment;
use std::collections::HashMap;

use objects::object::Object;
use objects::{SingleCharacter, Universe, BoxedObj};
use objects::universe;

use plugins::plugin::Plugin;
use plugins::next_object_result::NextObjectResult;
use plugins::default_plugin::DefaultPlugin;
use plugins::default_plugin;

type BuiltinsMap = universe::LocalsType;
type PluginsVec<'a> = Vec<&'a Plugin>;

#[derive(Debug)]
pub struct Parser<'a> {
   plugins: PluginsVec<'a>,
   builtins: BuiltinsMap,
}

#[derive(Debug)]
pub struct TokenPair<'a>(BoxedObj, &'a Plugin);

impl <'a> Parser <'a> {
	pub fn new() -> Parser<'a> {
		let mut res = Parser{ plugins: PluginsVec::new(), builtins: BuiltinsMap::new() };
      res.add_plugin(&default_plugin::INSTANCE);
      res
	}

   pub fn add_plugin(&mut self, plugin: &'a Plugin) {
      self.plugins.insert(0, plugin);
   }

   pub fn add_builtins(&mut self, builtins: BuiltinsMap) {
      unimplemented!();
   }

   pub fn process(&self, input: &str) -> Environment { // there are more thigns here that we dont need rn
      let stream = Universe::new();
      let universe = Universe::new();
      let mut env = Environment::new(stream, universe, self);
      let ref mut to_pass = Environment::new(Universe::new(), Universe::new(), self);
      for chr in input.chars() {
         env.stream.push( Box::new(SingleCharacter::new(chr)), to_pass );
      }
      self.parse(&mut env);
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





















