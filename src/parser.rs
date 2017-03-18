use objects::obj_rc::ObjRc;
use std::collections::HashMap;
use objects::object::Object;

use objects::single_character::SingleCharacter;
use objects::universe::Universe;
use objects::universe;
use result::{ObjError};

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
pub struct TokenPair(pub Result<ObjRc, ObjError>, pub &'static Plugin);

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
      let mut enviro = Universe::new();
      {
         for chr in input.chars() {
            stream.push( Box::new( SingleCharacter::new(chr) ));
         }
         self.parse(&mut stream, &mut enviro);
      }
      enviro
   }

   pub fn parse(&self,
                stream: &mut Universe,
                enviro: &mut Universe) {
      while !stream.stack.is_empty() {
         let TokenPair(token, plugin) = self.next_object(stream, enviro);
         match token {
            Ok(boxed_obj) => (*plugin).handle(boxed_obj, stream, enviro, self),
            Err(ObjError::EndOfFile) => break,
            Err(err) => panic!("Uncaught error: {:?}", err),
         }
      }
   }

   pub fn next_object(&self,
                      stream: &mut Universe,
                      enviro: &mut Universe) -> TokenPair {
      for pl in &(self.plugins) {
         match pl.next_object(stream, enviro, self) {
            PluginResponse::NoResponse => {},
            PluginResponse::Retry => return self.next_object(stream, enviro),
            PluginResponse::Response(obj) => return TokenPair(obj, *pl),
         }
      }
      TokenPair(Err(ObjError::EndOfFile), &default_plugin::INSTANCE)
   }

}





















