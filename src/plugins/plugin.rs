use parser::Parser;
use objects::universe::Universe;

use objects::boxed_obj::BoxedObj;
use result::ObjResult;
use std::fmt::Debug;

#[derive(Debug)]
pub enum PluginResponse {
   NoResponse,
   Retry,
   Response(ObjResult)
}

pub trait Plugin : Debug {
   fn next_object(&self,
                  stream: &mut Universe, // stream
                  enviro: &mut Universe, // enviro
                  parser: &Parser,       // parser
                 ) -> PluginResponse;
   fn handle(&self,
             token: BoxedObj,
             _: &mut Universe, // stream
             enviro: &mut Universe, // enviro
             _: &Parser,       // parser
            ) {
      enviro.push(token);
   }
}