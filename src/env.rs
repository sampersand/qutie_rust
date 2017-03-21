use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'a Parser,//&'a mut Parser<'a>,
}


impl <'a> Environment<'a> {
   pub fn new(stream: &'a mut Universe,
              universe: &'a mut Universe,
              parser: &'a Parser) -> Environment<'a> {
      Environment{
         stream: stream,
         universe: universe,
         parser: parser,
      }
   }
   pub fn fork<'o: 'n, 'n>(&'o mut self,
                           stream: Option<&'n mut Universe>,
                           universe: Option<&'n mut Universe>,
                           parser: Option<&'n Parser>
                          ) -> Environment<'n> {
      Environment::new(
         match stream {
            Some(obj) => obj,
            None => self.stream, //these might create memory leaks < why?? >
         },
         match universe {
            Some(obj) => obj,
            None => self.universe
         },
         match parser {
            Some(obj) => obj,
            None => self.parser,
         },
      )
   }
}
















