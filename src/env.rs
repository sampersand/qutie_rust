use parser::Parser;
use objects::universe::Universe;


/* Stable */
#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'a Parser,
}


/* Stable */
impl <'a> Environment<'a> {
   pub fn new(stream:   &'a mut Universe,
              universe: &'a mut Universe,
              parser:   &'a Parser ) -> Environment<'a> {
      Environment{
         stream: stream,
         universe: universe,
         parser: parser
      }
   }
   pub fn fork(self,
               stream: Option<&'a mut Universe>,
               universe: Option<&'a mut Universe>,
               parser: Option<&'a Parser>,
               ) -> [Environment<'a>; 2] {
      panic!()
   }
}
















