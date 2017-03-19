use parser::Parser;
use objects::universe::Universe;
use std::rc::Rc;

/* Stable */
#[derive(Debug)]
pub struct Environment<'a> {
    pub stream: Rc<&'a mut Universe>,
    pub universe: Rc<&'a mut Universe>,
    pub parser: Rc<&'a Parser>,
}


/* Stable */
impl <'a> Environment<'a> {
   pub fn new(stream: &'a mut Universe,
              universe: &'a mut Universe,
              parser: &'a Parser ) -> Environment<'a> {
      Environment{
         stream: Rc::new(stream),
         universe: Rc::new(universe),
         parser: Rc::new(parser)
      }
   }
   pub fn fork<'b: 'c, 'c>(&'b mut self,
               stream: Option<&'c mut Universe>,
               universe: Option<&'c mut Universe>,
               parser: Option<&'c Parser>,
              ) -> Environment<'c> {
      // Environment{
      //    stream: match stream {
      //       Some(obj) => Rc::new(obj),
      //       None => self.stream.clone(), //these might create memory leaks
      //    },
      //    universe: match universe {
      //       Some(obj) => Rc::new(obj),
      //       None => self.universe.clone()
      //    },
      //    parser: match parser {
      //       Some(obj) => Rc::new(obj),
      //       None => self.parser.clone(),
      //    },
      // }
      panic!()
   }
}
















