use environment::Environment;
use plugin::Plugin;
use std::collections::HashMap;

use objects::{Object, SingleCharacter, Universe};
use objects::universe;

type BuiltinsMap = universe::LocalsType;
type PluginsVec<'a> = Vec<&'a Plugin>;

#[derive(Debug)]
pub struct Parser<'a> {
   plugins: PluginsVec<'a>,
   builtins: BuiltinsMap,
}

impl <'a> Parser <'a> {
	pub fn new() -> Parser<'a> {
		Parser{ plugins: PluginsVec::new(), builtins: BuiltinsMap::new() }
	}

   pub fn add_plugin(&mut self, plugin: &'a Plugin) -> (){
      self.plugins.push(plugin);
   }

   pub fn add_builtins(&mut self, builtins: BuiltinsMap) -> () {
      unimplemented!();
   }

   pub fn process(&self, input: &str) -> Environment { // there are more thigns here that we dont need rn
      let mut env = Environment::new(Universe::new(), Universe::new(), self);
      for chr in input.chars() {
         // let a = SingleCharacter::new(chr);
         // println!("{:?}", a);
         // let b = Box::new( a );
         // println!("{:?}", b);
         // env.stream.push( b );
         // let c = Box::new( SingleCharacter::new( chr ));
         // let d = Box::new( SingleCharacter::new( chr ));
         // assert!(c == d);
         // println!("{:?}", env.stream.stack[0] == Box::new( SingleCharacter::new( chr )));
         // println!("---");
         env.stream.push(Box::new( SingleCharacter::new(chr) ));
      }
      self.parse(&mut env);
      env
   }

   fn parse(&self, env: &mut Environment){
      // println!("{:?}", env);

   }
}







