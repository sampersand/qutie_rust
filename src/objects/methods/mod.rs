macro_rules! no_method {
    ($meth:expr) => {{
      use objects::text::Text;
      use result::ObjError;
      Err(ObjError::NoSuchKey(rc!(Text::new($meth.to_string(), None))))
    }}
}

macro_rules! rc_meth {
    ($obj:expr, $func:path) => (rc!(BuiltinMethod::new($obj, $func)))
}

pub mod text_methods;