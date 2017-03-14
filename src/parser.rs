use environment::{Environment, OwnedEnvironment};
use std::collections::HashMap;

use objects::object::Object;
use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::boxed_obj::BoxedObj;
use objects::universe;

use plugins::plugin::Plugin;
use plugins::PluginResponse;
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
pub struct EOF;
pub type ParserNextObject = Result<TokenPair, EOF>;

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
      while !env.stream.stack.is_empty() {
         match self.next_object(env) {
            Ok(TokenPair(token, plugin)) => (*plugin).handle(token, env),
            Err(EOF) => break,
            Err(_) => panic!("Unknown parse return type")
         }
      }
   }
   pub fn next_object(&self, env: &mut Environment) -> ParserNextObject {
      for pl in &(self.plugins) {
         match pl.next_object(env) {
            PluginResponse::NoResponse => {},
            PluginResponse::Retry => { 
               return Ok(self.next_object(env));
            },
            PluginResponse::Response(obj) => {
               return Ok(TokenPair(obj, *pl));
            }
         }
      }
      Err(EOF)
      // panic!("No applicable plugin found for stream: {:?}", env.stream);
   }
}





















