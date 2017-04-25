use globals::IdType;
use env::Environment;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::rc::Rc;
use objects::text::Text;

use stream::Stream;
use objects::obj_rc::{ObjRc, ObjRcWrapper};
use objects::object::{Object, ObjType, ObjWrapper};
use objects::single_character::SingleCharacter;
use result::{ObjResult, ObjError, BoolResult};
use objects::boolean::{Boolean, BoolType};
use objects::number::Number;
use objects::symbol::Symbol;

pub type StackType = Vec<ObjRc>;
pub type LocalsType = HashMap<ObjRcWrapper, ObjRc>;
pub type GlobalsType = LocalsType;

#[derive(Clone, Copy, PartialEq)]
pub enum ParenType {
   Round, Square, Curly, Angled
}

impl ParenType {
   pub fn to_char(&self, side: bool) -> char {
      match *self {
         ParenType::Round => if side { ')' } else { '(' },
         ParenType::Square => if side { ']' } else { '[' },
         ParenType::Curly => if side { '}' } else { '{' },
         ParenType::Angled => if side { '>' } else { '<' }
      }
   }
   pub fn from_char(inp: char, side: bool) -> Option<ParenType> {
      match inp {
         '(' if !side => Some(ParenType::Round),
         ')' if  side => Some(ParenType::Round),
         '[' if !side => Some(ParenType::Square),
         ']' if  side => Some(ParenType::Square),
         '{' if !side => Some(ParenType::Curly),
         '}' if  side => Some(ParenType::Curly),
         '<' if !side => Some(ParenType::Angled),
         '>' if  side => Some(ParenType::Angled),
         _ => None
      }
   }
}


#[allow(dead_code)]
pub struct Universe {
   id: IdType,
   pub parens: (ParenType, ParenType),
   pub stack: StackType,
   pub locals: LocalsType,
   pub globals: GlobalsType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessType {
   Stack,
   Locals,
   Globals,
   All,
   NonStack
}

/* initializer and representation */
impl Universe {
   pub fn new(parens: Option<(ParenType, ParenType)>,
              stack: Option<StackType>,
              locals: Option<LocalsType>,
              globals: Option<GlobalsType>) -> Universe {
      Universe{
         id: next_id!(),
         parens: 
            if let Some(obj) = parens { obj } 
            else { (ParenType::Angled, ParenType::Angled) },
         stack: 
            if let Some(obj) = stack { obj }
            else { StackType::new() },
         locals: 
            if let Some(obj) = locals { obj } 
            else { LocalsType::new() },
         globals:
            if let Some(obj) = globals { obj }
            else { GlobalsType::new() },
      }
   }

   pub fn to_rc(self) -> Rc<Universe>{
      Rc::new(self)
   }

   pub fn parse_str(input: &str) -> StackType {
      let mut stack = StackType::new();
      for c in input.chars() {
         stack.push(SingleCharacter::new(c).to_rc())
      }
      stack
   }

   pub fn to_globals(&self) -> Universe {
      let mut globals = self.globals.clone();
      globals.extend(self.locals.clone());
      Universe::new(Some(self.parens), None, None, Some(globals))
   }

   fn to_stream(&self) -> Option<Stream> {
      let mut stream_acc = String::new();
      for item in &self.stack {
         if !item.is_a(ObjType::SingleCharacter) {
            return None;
         }
         stream_acc.push(cast_as!(CL; item, SingleCharacter).char_val);
      }
      Some(Stream::from_str(stream_acc.as_str()))
   }
   pub fn merge_vars(&mut self, other: Rc<Universe>) {
      for (key, val) in other.locals.iter() {
         self.locals.insert(key.clone(), val.clone());
      }
      for (key, val) in other.globals.iter() {
         self.locals.insert(key.clone(), val.clone());
      }
   }
   fn to_string(&self) -> String {
      let mut ret = self.parens.0.to_char(false).to_string();
      if let Some(stream) = self.to_stream(){
         ret.push_str(stream.to_raw_string().as_str());
      } else {
         for obj in &self.stack{
            ret.push_str(obj.to_string().as_str());
            ret.push_str(", ");
         }
         if !self.stack.is_empty() {
            assert_eq!(ret.pop().expect("can't pop off ret string (pos 1)"), ' ');
            assert_eq!(ret.pop().expect("can't pop off ret string (pos 2)"), ',');
         }
         for (key, val) in self.locals.iter() {
            ret.push_str(key.0.to_string().as_str());
            ret.push_str(": ");
            ret.push_str(val.to_string().as_str());
            ret.push_str(", ");
         }
         if !self.locals.is_empty() {
            assert_eq!(ret.pop().expect("can't pop off ret string (pos 3)"), ' ');
            assert_eq!(ret.pop().expect("can't pop off ret string (pos 4)"), ',');
         }
      }
      ret.push(self.parens.1.to_char(true));
      ret
   }
}

/* Use as a stack */
impl Universe {
   pub fn pop(&mut self) -> ObjResult {
      if let Some(obj) = self.stack.pop() {
         Ok(obj)
      } else {
         Err(ObjError::EndOfFile)
      }
   }

   pub fn push(&mut self, other: ObjRc) {
      self.stack.push(other);
   }
}

/* Accessors */
impl Universe {
   fn get_atype(&self, key: &ObjRc, a_type: AccessType) -> AccessType {
      match a_type {
         AccessType::All => 
            if key.is_a(ObjType::Number) {
               AccessType::Stack
            } else {
               self.get_atype(key, AccessType::NonStack)
            },
         AccessType::NonStack =>
            if self.locals.contains_key(&ObjRcWrapper(key.clone()))   {
               AccessType::Locals
            } else {
               AccessType::Globals
            },
         o @ _ => o
      }
   }

   pub fn get(&self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      match self.get_atype(&key, a_type) {
         AccessType::Stack => 
            if let Some(obj) = self.stack.get(cast_as!(CL; key, Number).num_val as usize) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key))
            },
         AccessType::Locals => 
            if let Some(obj) = self.locals.get(&ObjRcWrapper(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         AccessType::Globals => 
            if let Some(obj) = self.globals.get(&ObjRcWrapper(key)) {
               Ok(obj.clone())
            } else {
               Err(ObjError::NoSuchKey(key_clone))
            },
         o @ _ => panic!("Unknown a_type: {:?}", o)
      }
   }

   pub fn set(&mut self, key: ObjRc, val: ObjRc, a_type: AccessType) {
      match self.get_atype(&key, a_type) {
         AccessType::Stack => 
            {
               let stack_len = self.stack.len();
               let pos = cast_as!(key, Number).num_val as isize;
               let pos =
                  if pos < 0 {
                     stack_len as isize + pos /* if we have a negative position, invert it */
                  } else {
                     pos
                  } as usize;

               if stack_len < pos { /* if we access an element too far out, add nulls until we get there */
                  for _ in stack_len..(pos - 1) {
                     self.stack.push(new_obj!(BOOL_STATIC, Null))
                  }
                  self.stack.push(val);
               } else {
                  self.stack.push(val);
                  if pos != stack_len {
                     self.stack.swap_remove(pos);
                  }
               }
         },
         AccessType::Locals => 
            {
               self.locals.insert(ObjRcWrapper(key), val);
            },
         AccessType::Globals =>
            {
               self.globals.insert(ObjRcWrapper(key), val);
            },
         o @ _ => panic!("Shouldn't be trying to set type: {:?}", o)
      };
   }
   pub fn del(&mut self, key: ObjRc, a_type: AccessType) -> ObjResult {
      let key_clone = key.clone();
      let ret =
         match a_type {
            AccessType::Locals => self.locals.remove(&ObjRcWrapper(key)),
            AccessType::Globals => self.globals.remove(&ObjRcWrapper(key)),
            _ => unimplemented!()
         };
      if let Some(obj) = ret {
         Ok(obj)
      } else {
         Err(ObjError::NoSuchKey(key_clone))
      }
   }

   pub fn call(&self, args: Rc<Universe>, env: &mut Environment, do_pop: bool) -> ObjResult {
      let mut new_universe = args.to_globals();
      let mut stream = &mut self.to_stream().expect("can't turn into a stream");
      new_universe.locals.insert(ObjRcWrapper(new_obj!(SYM_STATIC, "__args")), args.clone()); /* add __args in */
      {
         let cloned_env = env.parser.clone();
         let mut new_env = &mut env.fork(Some(stream), Some(&mut new_universe), None);
         cloned_env.parse(new_env);
      }

      Ok(if do_pop {
            if let Some(obj) = new_universe.stack.pop() { obj }
            else { new_obj!(BOOL_STATIC, Null) }
         } else { new_universe.to_rc() })
   }

   pub fn exec_all(&self, env: &mut Environment) -> ObjResult {
      let mut new_stream = self.to_stream().expect("can't make stream");
      let cloned_env = env.parser.clone();
      {
         cloned_env.parse(&mut env.fork(Some(&mut new_stream), None, None));
      }
      Ok(new_obj!(BOOL_STATIC, Null))
   }


   pub fn exec(&self, env: &mut Environment) -> ObjResult {
      let mut new_universe = env.universe.to_globals();
      let mut new_stream = self.to_stream().expect("can't make stream");
      let cloned_env = env.parser.clone();
      {
         cloned_env.parse(&mut env.fork(Some(&mut new_stream), Some(&mut new_universe), None));
      }
      Ok(new_universe.to_rc())
   }

   fn replace(&self, other: Rc<Universe>) {
      let mut me: &mut Universe = unsafe {
         use std::mem::transmute;
          #[allow(mutable_transmutes)]
         transmute(self)
      };
      me.parens = other.parens.clone();
      me.stack = other.stack.clone();
      me.locals = other.locals.clone();
      me.globals = other.globals.clone();
   }
}
impl Clone for Universe{
   fn clone(&self) -> Universe{
      Universe::new(Some(self.parens.clone()),
                    Some(self.stack.clone()),
                    Some(self.locals.clone()),
                    Some(self.globals.clone()))
   }
}

macro_rules! universe_method {
   (TYPE; $name:ident, $ret_type:ident, $usr_fn_name:expr, $ret_fn:expr) => {
      fn $name(&self, env: &mut Environment) -> Result<Rc<$ret_type>, ObjError> {
         let self_rc = self.clone().to_rc();
         match get_method!(self_rc.clone(), $usr_fn_name, env) {
            Ok(obj) => {
               let ret = obj.qt_call(env.universe.to_globals().to_rc(), env).
                             expect("call returned err");
               self.replace(self_rc);
               Ok(cast_as!(ret, $ret_type))
            },
            Err(_) => $ret_fn(self)
         }
      }
   };
   (OPER_BOOL; $name:ident, $usr_fn_name:expr) => {
      fn $name(&self, other: ObjRc, env: &mut Environment) -> BoolResult {
         let self_rc = self.clone().to_rc();
         match get_method!(self_rc.clone(), $usr_fn_name, env) {
            Ok(method) => {
               let mut uni = env.universe.to_globals();
               uni.push(other);
               let ret = method.qt_call(Rc::new(uni), env).expect("call returned error for oper");
               self.replace(self_rc);
               Ok(cast_as!(ret, Boolean))
            },
            Err(ObjError::NoSuchKey(_)) => Err(ObjError::NotImplemented),
            Err(err) => unreachable!("is this really unreachable? {:?}", err) 
         }
      }
   };

   (OPER; $name:ident, $usr_fn_name:expr) => {
      fn $name(&self, other: ObjRc, env: &mut Environment) -> ObjResult {
         let self_rc = self.clone().to_rc();
         match get_method!(self_rc.clone(), $usr_fn_name, env) {
            Ok(method) => {
               let mut uni = env.universe.to_globals();
               uni.push(other);
               assert_debug!(is_a; method, UserFunction);
               let ret = method.qt_call(Rc::new(uni), env).expect("call returned error for oper");
               self.replace(self_rc);
               Ok(ret)
            },
            Err(ObjError::NoSuchKey(_)) => Err(ObjError::NotImplemented),
            Err(err) => unreachable!("is this really unreachable? {:?}", err) 
         }
      }
   }
}

/* QT things */
impl Object for Universe {
   impl_defaults!(OBJECT; Universe);
   universe_method!(TYPE; qt_to_text, Text, "__text",
                    (|me: &Universe| Ok(new_obj!(TEXT, me.to_string()))));
   universe_method!(TYPE; qt_to_bool, Boolean, "__bool",
                    (|me: &Universe| Ok(new_obj!(BOOL, me.stack.is_empty() && me.locals.is_empty()))));
   universe_method!(TYPE; qt_to_num, Number, "__num",
                    (|_: &Universe| Err(ObjError::NotImplemented)));
   universe_method!(OPER; qt_add_l, "__add"); universe_method!(OPER; qt_add_r, "__add_r");
   universe_method!(OPER; qt_sub_l, "__sub"); universe_method!(OPER; qt_sub_r, "__sub_r");
   universe_method!(OPER; qt_mul_l, "__mul"); universe_method!(OPER; qt_mul_r, "__mul_r");
   universe_method!(OPER; qt_div_l, "__div"); universe_method!(OPER; qt_div_r, "__div_r");
   universe_method!(OPER; qt_mod_l, "__mod"); universe_method!(OPER; qt_mod_r, "__mod_r");
   universe_method!(OPER; qt_pow_l, "__pow"); universe_method!(OPER; qt_pow_r, "__pow_r");

   universe_method!(OPER_BOOL; qt_eql_l, "__eql"); universe_method!(OPER_BOOL; qt_eql_r, "__eql_r");
   universe_method!(OPER_BOOL; qt_neq_l, "__neq"); universe_method!(OPER_BOOL; qt_neq_r, "__neq_r");
   universe_method!(OPER_BOOL; qt_lth_l, "__lth"); universe_method!(OPER_BOOL; qt_lth_r, "__lth_r");
   universe_method!(OPER_BOOL; qt_gth_l, "__gth"); universe_method!(OPER_BOOL; qt_gth_r, "__gth_r");
   universe_method!(OPER_BOOL; qt_leq_l, "__leq"); universe_method!(OPER_BOOL; qt_leq_r, "__leq_r");
   universe_method!(OPER_BOOL; qt_geq_l, "__geq"); universe_method!(OPER_BOOL; qt_geq_r, "__geq_r");
   universe_method!(OPER_BOOL; qt_cmp_l, "__cmp"); universe_method!(OPER_BOOL; qt_cmp_r, "__cmp_r");
   universe_method!(OPER; qt_rgx_l, "__rgx"); universe_method!(OPER; qt_rgx_r, "__rgx_r");
   // universe_method!(OPER; qt_set_l, "__set");
   // universe_method!(OPER; qt_get_l, "__get");


   fn qt_exec(&self, env: &mut Environment) -> ObjResult {
      self.exec(env)
   }

   fn qt_get_l(&self, key: ObjRc, _: &mut Environment) -> ObjResult {
      self.get(key, AccessType::All)
   }

   fn qt_set_l(&mut self, key: ObjRc, val: ObjRc, _: &mut Environment) -> ObjResult {
      self.set(key, val.clone(), AccessType::All);
      Ok(val)
   }

   fn qt_call(&self, args: ObjRc, env: &mut Environment) -> ObjResult {
      self.call(cast_as!(args, Universe), env, true)
   }
}


impl Display for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      try!(write!(f, "--[ Stack ]--\n"));
      for (i, ele) in self.stack.iter().enumerate() {
         try!(write!(f, "\t{:?}. {:?}\n", i, ele));
      }
      // write!(f, "--[ Locals ]--\n");
      // for (key, val) in self.locals.iter() {
      //    match val.old_obj_type(){
      //       OldObjType::Operator(_) => {},
      //       _ => { write!(f, "\t{:?}: {:?}\n", key, val); }
      //    };
      // }
      Ok(())
   }
}
impl Debug for Universe {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      try!(write!(f, "U("));
      if self.stack.len() > 10 {
         try!(write!(f, "[...], "))
      } else {
         try!(write!(f, "{:?}, ", self.stack))
      }
      // use std::iter::Iterator;
      // let tmp = self.locals.clone();
      // let locals = tmp.values().filter(|v| !v.is_a(ObjType::Builtin));

      if self.locals.len() > 5 {
         try!(write!(f, "{{ ... }}"))
      } else {
         try!(write!(f, "{:?}", self.locals))
      }
      write!(f, ")")
   }
}














