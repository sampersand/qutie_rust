macro_rules! obj_functions {
   (QT_TO_BOOL; $bool_expr:expr) => {
      fn qt_to_bool(&self, _: &mut Environment) -> Result<Rc<Boolean>, ObjError> {
         let ans = ($bool_expr)(self); /* is a closure, for now. Later on i'll figure out how to fix that */
         Ok(new_obj!(BOOL, ans))
      }
   };
   (QT_TO_TEXT) => {
      fn qt_to_text(&self, _: &mut Environment) -> Result<Rc<Text>, ObjError> {
         Ok(new_obj!(TEXT, self.to_string()))
      }
   };
   (QT_EQL; $comp_item:ident) => {
      fn qt_eql_l(&self, other: ObjRc, _: &mut Environment) -> ObjResult {
         Ok(rc!(Boolean::from(self.obj_type() == other.obj_type() &&
                              self.$comp_item == cast_as!(CL; other, Self).$comp_item)))
      }
      fn qt_eql_r(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
         self.qt_eql_l(other, env)
      }
   };
   (QT_METHODS; $obj_mod:ident) => {
      fn qt_method(&self, meth: &str, env: &mut Environment) -> ObjResult {
         use objects::methods::$obj_mod;
         $obj_mod::get_method(self, meth, env)
      }
   };
   (OBJ_TYPE; $name:ident ) => {
      fn obj_type(&self) -> ObjType { ObjType::$name }
   };
   (SOURCE; $name:ident ) => {
      fn source(&self) -> Vec<SingleCharacter> {
         let mut ret = vec![];
         for chr in self.to_string().chars(){
            ret.push(SingleCharacter::new(chr));
         }
         ret
      }
   }

}
macro_rules! impl_defaults {
   (OBJECT; $name:ident ) => {
      fn obj_type(&self) -> ObjType { ObjType::$name }
      fn source(&self) -> Vec<SingleCharacter> {
         let mut ret = vec![];
         for chr in self.to_string().chars(){
            ret.push(SingleCharacter::new(chr));
         }
         ret
      }
   };
   (DISPLAY_DEBUG; $name:ty, $chr:expr) => {
      use std::fmt::{Debug, Formatter, Error, Display};
      impl Display for $name{
         fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "{}", self.to_string())
         }
      }

      impl Debug for $name{
         fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "{}({})", $chr, self)
         }
      }
   }
}

macro_rules! new_obj {
   (SYM, $name:expr) => ( rc!(Symbol::from($name)) );
   (SYM_STATIC, $name:expr) => ( rc!(Symbol::new($name)) );
   (TEXT, $name:expr) => ( rc!(Text::new($name, None)) );
   (TEXT_STATIC, $name:expr) => ( rc!(Text::from($name)) );
   (NUM, $name:expr) => ( rc!(Number::new($name)) );
   (BOOL, $name:expr) => ( rc!(Boolean::from($name)) );
}

macro_rules! to_type {
   (STRING; $inp:expr, $env:expr) => ( $inp.qt_to_text($env).unwrap().text_val.clone() );
   (BOOL; $inp:expr, $env:expr) => ( $inp.qt_to_bool($env).unwrap().bool_val );
   (NUM;  $inp:expr, $env:expr) => ( $inp.qt_to_num($env).unwrap().num_val );
}


macro_rules! rc {
   ($imp:expr) => ( Rc::new($imp) )
}

macro_rules! map {
   { TYPE; $global_type:ident, $($key:expr => $value:expr),+ } => {
      {
         let mut m = $global_type::new();
         $(
            m.insert(ObjRcWrapper(new_obj!(SYM_STATIC, $key)), $value);
         )+
         m
      }
   }
}

macro_rules! cast_as {
    ($from:expr, $to:ident) => (ObjWrapper::<$to>::from($from).0);
    (CL; $from:expr, $to:ident) => ( cast_as!($from.clone(), $to) )
}

macro_rules! get_method {
   ($obj:expr, $meth:expr, $env:expr) => {{
      use objects::operator::get_fn;
      get_fn(Some($obj), Some(new_obj!(SYM_STATIC, $meth)), $env)
   }};
   (CL; $obj:expr, $meth:expr, $env:expr) => ( get_method!($obj.clone(), $meth, $env) )
}








