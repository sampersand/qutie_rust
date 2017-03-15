use environment::Environment;
use std::collections::HashMap;

use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::universe;
use result::{ObjResult, ObjErr};

use plugins::plugin::Plugin;
use plugins::plugin::PluginResponse;
use plugins::default_plugin::DefaultPlugin;
use plugins::default_plugin;


type BuiltinsMap = universe::LocalsType;
type PluginsVec = Vec<&'static Plugin>;

#[derive(Debug)]
pub struct Parser {
   plugins: PluginsVec,
   builtins: BuiltinsMap, /* rn, pointless */
}

#[derive(Debug)]
pub struct TokenPair(pub ObjResult, pub &'static Plugin);

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

   pub fn process(&self, input: &str) -> Universe {
      let mut stream = Universe::new();
      let mut universe = Universe::new();
      {
         let mut env = Environment::new(&mut stream, &mut universe, self);
         for chr in input.chars() {
            env.stream.push( Box::new( SingleCharacter::new(chr) ));
         }
         self.parse(&mut env);
      }
      universe
   }

   pub fn parse(&self, env: &mut Environment) {
      while !env.stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(env);
         match token {
            Ok(boxed_obj) => (*plugin).handle(boxed_obj, env),
            Err(ObjErr::EndOfFile) => break,
            Err(err) => panic!("Uncaught error: {:?}", err),
         }
      }
   }

   pub fn next_object(&self, env: &mut Environment) -> TokenPair {
      for pl in &(self.plugins) {
         match pl.next_object(env) {
            PluginResponse::NoResponse => {},
            PluginResponse::Retry => return self.next_object(env),
            PluginResponse::Response(obj) => return TokenPair(obj, *pl),
         }
      }
      TokenPair(Err(ObjErr::EndOfFile), &default_plugin::INSTANCE)
   }

}





















