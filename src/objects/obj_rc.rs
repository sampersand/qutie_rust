use globals;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::ops::Deref;
use objects::object::Object;
use result::ObjError;
use std::fmt::{Display, Formatter, Error, Debug};


pub type ObjRc = Rc<Object>;

#[derive(Clone, Debug)]
pub struct ObjRcWrapper(pub ObjRc);

impl PartialEq for ObjRcWrapper {
   fn eq(&self, other: &ObjRcWrapper) -> bool {
      use env::Environment;
      use stream::Stream;
      use parser::Parser;
      use objects::universe::Universe;
      let stream = &mut Stream::new(vec![]);
      let uni = &mut Universe::new(None, None, None, None);
      let parser = &mut Parser::new();
      let rc = Rc::new(parser);
      let mut env = Environment::new(stream, uni, rc);
      // println!("trying to compare {:?} to {:?}", self.0, other.0);
      let ret = (*self.0)._eql( other.clone().0, &mut env );
      // println!("compared {:?} to {:?}: {:?}", self.0, other.0, ret);
      ret
   }
}
impl Eq for ObjRcWrapper{}
impl Hash for ObjRcWrapper{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      // todo: hash
      hasher.write(&[1]);
      // (*self).hash(hasher)
   }
}





