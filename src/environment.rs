use parser::Parser;
use objects::universe::Universe;


#[derive(Debug)]
pub struct OwnedEnvironment<'a> {
   pub stream: Universe,
   pub universe: Universe,
   pub parser: &'a Parser,
}

impl <'a> OwnedEnvironment<'a> {
   pub fn to_unowned(&mut self) -> Environment {
      Environment::new(&mut self.stream, &mut self.universe, self.parser)
   }
   pub fn new(stream: Universe, universe: Universe, parser: &'a Parser) -> OwnedEnvironment<'a> {
      OwnedEnvironment{ stream: stream, universe: universe, parser: parser }
   }
}

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a mut Universe,
    pub universe: &'a mut Universe,
    pub parser: &'a Parser,
}


impl <'a> Environment<'a> {
   pub fn new(stream: &'a mut Universe,
              universe: &'a mut Universe,
              parser: &'a Parser) -> Environment<'a> {
      Environment{ stream: stream, universe: universe, parser: parser }
   }
}
















