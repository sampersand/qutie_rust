use parser::Parser;
use objects::Universe;

#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: Universe,
    pub universe: Universe,
    pub parser: &'a Parser<'a>,
}
use std;
impl <'a> Environment<'a> {
   pub fn new(stream: Universe, universe: Universe, parser: &'a Parser<'a>) -> Environment<'a> {
      Environment{ stream: stream, universe: universe, parser: parser }
   }
}
















