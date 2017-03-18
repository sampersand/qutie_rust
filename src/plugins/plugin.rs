use objects::obj_rc::ObjRc;
use parser::Parser;
use objects::universe::Universe;

use objects::object::Object;
use result::{ObjResult, ObjError};
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
             token: ObjRc,
             _: &mut Universe, // stream
             enviro: &mut Universe, // enviro
             _: &Parser,       // parser
            ) {
      enviro.push(token);
   }
}