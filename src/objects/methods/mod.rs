macro_rules! no_method {
    ($meth:expr) => {{
      use objects::text::Text;
      use result::ObjError;
      Err(ObjError::NoSuchKey(rc_obj!(TEXT; $meth.to_string())))
    }}
}

macro_rules! rc_meth {
    ($obj:expr, $func:path) => (rc!(BuiltinMethod::new($obj, $func)))
}

pub mod text_methods;