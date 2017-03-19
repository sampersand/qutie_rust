use parser::Parser;
use objects::universe::Universe;

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'a Parser,
}


impl <'a> Environment<'a> {
   pub fn new(stream: &'a mut Universe,
              universe: &'a mut Universe,
              parser: &'a Parser ) -> Environment<'a> {
      Environment{
         stream: stream,
         universe: universe,
         parser: parser,
      }
   }
   pub fn fork<'b: 'c, 'c>(&'b mut self,
               stream: Option<&'c mut Universe>,
               universe: Option<&'c mut Universe>,
               parser: Option<&'c Parser>,
              ) -> Environment<'c> {
      Environment{
         stream: match stream {
            Some(obj) => obj,
            None => self.stream, //these might create memory leaks
         },
         universe: match universe {
            Some(obj) => obj,
            None => self.universe
         },
         parser: match parser {
            Some(obj) => obj,
            None => self.parser,
         },
      }
   }
}
















