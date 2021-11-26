#![allow(unused)]

#[deprecated(
	since = "0.2.0",
	note = "Use cstr instead."
)]
#[macro_export]
macro_rules! cstring {
	($rstring:expr) => {
		{
			let v = std::ffi::CString::new($rstring);
			v.expect("Couldn't make CString from rust string").as_ptr()
		}
	}
}

// Creates *const i8 from a rust string literal by concatting a \0 to the end of it.
#[macro_export]
macro_rules! cstr {
	($rstring:expr) => {
		concat!($rstring, "\0").as_ptr() as *const i8
	}
}

// Get a rust &str from a const char*
#[macro_export]
macro_rules! rstr {
	($cstring:expr) => {
		{
			#[allow(unused_unsafe)]
			let cstr = unsafe{ std::ffi::CStr::from_ptr($cstring) };
			cstr.to_str().expect("Couldn't unwrap CString")
		}
	}
}

#[macro_export]
#[deprecated(since = "0.5.0", note = "Use rstr instead.")]
macro_rules! rstring {
	($a:tt) => { rstr!($a) }
}

#[allow(unused_macros)]
#[macro_export]
/// Like println!, however it prints to the gmod server's console.
// First arg is the lua state.
// Rest are varargs.
// Can be either a variable storing a str literal, or a referenced String / str variable
macro_rules! printgm {
	($state:expr, $($x:expr),*) => {
		{
			let printargs = format!( $($x,)* );
			if let Ok(fmt) = std::ffi::CString::new(printargs) {
				rglua::lua_shared::lua_getglobal( $state, cstr!("print") );
				rglua::lua_shared::lua_pushstring( $state, fmt.as_ptr() );
				rglua::lua_shared::lua_call( $state, 1, 0 );
			}
		}
	};
}