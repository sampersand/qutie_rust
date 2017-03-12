use parser::Parser;
use objects::universe::Universe;

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a Universe,
    pub universe: &'a Universe,
    pub parser: &'a Parser<'a>,
}
use std;
impl <'a> Environment<'a> {
   pub fn new(stream: &'a Universe,
              universe: &'a Universe,
              parser: &'a Parser<'a>) -> Environment<'a> {
      Environment{ stream: stream, universe: universe, parser: parser }
   }
   pub fn fork(&self,
               stream: Option<&Universe>,
               universe: Option<&Universe>,
               parser: Option<&Parser>) -> Environment {
      Environment::new(&Universe::new(), &Universe::new(), self.parser)
   }
}
















