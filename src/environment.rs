use parser::Parser;
use objects::universe::Universe;


/* Stable */
#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: &'a mut Universe<'a>,
    pub universe: &'a mut Universe<'a>,
    pub parser: &'a Parser<'a>,
}


/* Stable */
impl <'a> Environment<'a> {
   pub fn new(stream:   &'a mut Universe<'a>,
              universe: &'a mut Universe<'a>,
              parser:   &'a Parser<'a> ) -> Environment<'a> {
      Environment{
         stream: stream,
         universe: universe,
         parser: parser
      }
   }
}
















