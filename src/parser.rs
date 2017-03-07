use environment::Environment;
use std::collections::HashMap;
use util;

use objects::{Object, SingleCharacter, Universe, BoxedObj};
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

pub struct TokenPair<'a>(BoxedObj, &'a Plugin);

impl <'a> Parser <'a> {
	pub fn new() -> Parser<'a> {
		let mut res = Parser{ plugins: PluginsVec::new(), builtins: BuiltinsMap::new() };
      res.add_plugin(&default_plugin::INSTANCE);
      res
	}

   pub fn add_plugin(&mut self, plugin: &'a Plugin) -> (){
      self.plugins.push(plugin);
   }

   pub fn add_builtins(&mut self, builtins: BuiltinsMap) -> () {
      unimplemented!();
   }

   pub fn process(&self, input: &str) -> Environment { // there are more thigns here that we dont need rn
      let stream = Universe::new();
      let universe = Universe::new();
      let mut env = Environment::new(stream, universe, self);
      for chr in input.chars() {
         env.stream.push(Box::new( SingleCharacter::new(chr) ));
      }
      self.parse(&mut env);
      env
   }
   pub fn parse(&self, env: &mut Environment){
      while !env.stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(env);
         (*plugin).handle(token, env);
         break;

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
      println!("No applicable plugin found for stream: {:?}", env.stream);
      util::exit(1);
   }
}





















