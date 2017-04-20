macro_rules! assert_debug {
   ($cond:expr) => ( assert_debug!($cond, "assertion failed") );
   ($cond:expr, $msg:expr) => (
      if true {
         assert!($cond, $msg);
      } else {
         $cond;
      }
   );
   (is_a; $inp:expr, $obj_type:ident) => ( assert_debug!($inp.is_a(ObjType::$obj_type)) );
   (eq; $lhs:expr, $rhs:expr) => ( assert_debug!(eq; $lhs, $rhs, "assertion failed") );
   (eq; $lhs:expr, $rhs:expr, $msg:expr) => ( assert_debug!($lhs == $rhs, $msg) );
   (none; $inp:expr) => ( assert_debug!($inp.is_none()) )
}