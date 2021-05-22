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

// Get a rust &str from a const char*
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

#[allow(unused_macros)]
#[macro_export]
/// Like println!, however it prints to the gmod server's console.
macro_rules! printgm {
    // First arg is the lua state.
    // Rest are varargs.
    // Can be either a variable storing a str literal, or a referenced String / str variable
    ($state:expr, $($x:expr),*) => {
        {
            let stmt = format!( $($x,)* ); // Everything past the state will be as if it were inside a format! call.
            rglua::lua_getglobal!($state, rglua::cstring!("print") );
            rglua::lua_shared::lua_pushstring($state, rglua::cstring!(stmt) );
            // 1 arg, 0 results
            rglua::lua_shared::lua_call($state, 1, 0);
        }
    };
}