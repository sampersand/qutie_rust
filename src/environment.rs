use parser::Parser;
use objects::Universe;

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: Universe,
    pub universe: Universe,
    pub parser: &'a Parser<'a>,
}

impl <'a> Environment<'a> {
   pub fn new(stream: Universe, universe: Universe, parser: &'a Parser<'a>) -> Environment<'a> {
      Environment{ stream: stream, universe: universe, parser: parser }
   }
   // pub fn fork_stream(stream: Universe)
}