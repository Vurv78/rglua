#![allow(unused)]

// Get a const char* from a &str
#[macro_export]
macro_rules! cstring {
    ($rstring:expr) => {
        {
            let v = std::ffi::CString::new($rstring);
            v.expect("Couldn't make CString from rust string").as_ptr()
        }
    }
}

// Get a rust string from a const char*
#[macro_export]
macro_rules! rstring {
    ($cstring:expr) => {
        {
            #[allow(unused_unsafe)]
            let cstr = unsafe{ std::ffi::CStr::from_ptr($cstring) };
            cstr.to_str().expect("Couldn't unwrap CString")
        }
    }
}