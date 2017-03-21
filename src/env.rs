use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment<'parser: 'a, 'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'parser mut Parser<'parser>,//&'a mut Parser<'a>,
}


impl <'parser: 'a, 'a> Environment<'parser, 'a> {
   pub fn new(stream: &'a mut Universe,
              universe: &'a mut Universe,
              parser: &'parser mut Parser<'parser>) -> Environment<'parser, 'a> {
      Environment{
         stream: stream,
         universe: universe,
         parser: parser,
      }
   }
   pub fn fork<'o: 'n, 'n>(&'o mut self,
               stream: Option<&'n mut Universe>,
               universe: Option<&'n mut Universe>,
               parser: Option<&'parser mut Parser<'parser>>
              ) -> Environment<'parser, 'n> {
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
            None => &mut self.parser.clone(),
         },
      )
   }
}
















