#![allow(unused)]

/// Creates *const i8 from a &str
/// This either takes a literal and appends a null char (\0) to it.
/// or if it is a value, makes a cstring and returns the pointer to it.
/// Will panic if passed an expression that a CString could not be created from.
/// # Examples
/// ```rust
/// let a = b"Hello world!".as_ptr() as *const i8;
/// let b = cstr!("Hello world!");
/// assert_eq!(*a, *b);
/// ```
#[macro_export]
macro_rules! cstr {
	($rstring:literal) => {
		concat!($rstring, "\0").as_ptr() as *const i8
	};
	($rstring:expr) => {{
		let cstr = CString::new($rstring).expect("Couldn't make CString from rust string");
		cstr.as_ptr()
	}};
}

/// Tries to create a *const i8 from a &str
/// This either takes a literal and appends a null char (\0) to it.
/// or if it is a value, makes a cstring and returns the pointer to it.
/// # Examples
/// ```rust
/// let a = b"Hello world!".as_ptr() as *const i8;
/// let b = try_cstr!("Hello world!");
/// assert_eq!(*a, *b.unwrap());
/// ```
#[macro_export]
macro_rules! try_cstr {
	($rstring:literal) => {
		concat!($rstring, "\0").as_ptr() as *const i8
	};
	($rstring:expr) => {{
		let cstr = CString::new($rstring);
		cstr.map(|cstr| cstr.as_ptr())
	}};
}

/// Tries to convert a const char* to a &str
/// Will panic if the const char* is not valid utf-8
/// # Examples
/// ```rust
/// let cstr = cstr!("Hello World");
/// let rust_str = rstr!(cstr);
/// assert_eq!(rust_str, "Hello World");
/// ```
#[macro_export]
macro_rules! rstr {
	($cstring:expr) => {{
		#[allow(unused_unsafe)]
		let cstr = unsafe { std::ffi::CStr::from_ptr($cstring) };
		cstr.to_str().expect("Couldn't unwrap CString")
	}};
}

#[macro_export]
/// Tries to convert a const char* to an &str
/// # Examples
/// ```rust
/// let cstr = cstr!("Hello World");
/// let rstr = try_rstr!(cstr);
/// assert(rstr.is_ok()); // Should be perfectly valid to convert to utf8
/// ```
macro_rules! try_rstr {
	($cstring:expr) => {{
		#[allow(unused_unsafe)]
		let cstr = unsafe { std::ffi::CStr::from_ptr($cstring) };
		cstr.to_str()
	}};
}

#[allow(unused_macros)]
#[macro_export]
/// Like println!, however it prints to the gmod server's console.
/// First arg is the lua state.
/// Rest are varargs.
/// Can be either a variable storing a str literal, or a referenced String / str variable
/// # Examples
/// ```rust
/// printgm!(state, "Hello {}!", "world");
/// ```
macro_rules! printgm {
	($state:expr, $($x:expr),*) => {
		{
			let printargs = format!( $($x,)* );
			if let Ok(fmt) = std::ffi::CString::new(printargs) {
				rglua::lua_shared::lua_getglobal( $state, rglua::cstr!("print") );
				rglua::lua_shared::lua_pushstring( $state, fmt.as_ptr() );
				rglua::lua_shared::lua_call( $state, 1, 0 );
			}
		}
	};
}

/// Creates an array of LuaRegs for you to be used with luaL_register
/// # Examples
/// Basic usage
/// ```rust
/// let my_library = reg! [
/// 	"max" = max,
/// 	"min" = min,
/// ];
/// ```
/// Returns a &[crate::types::LuaReg]
#[macro_export]
macro_rules! reg {
	( $( $name:expr => $func:expr ),* ) => {
		&[ $( rglua::types::LuaReg { name: rglua::cstr!($name), func: Some($func) } ),*, rglua::types::LuaReg { name: std::ptr::null(), func: None } ]
	};
}

use crate::types::LuaState;
/// Prints out the current state of the lua stack without affecting it.
/// Comes out in this format:
/// ```text
/// [1] 'number' = 5000
/// [2] 'string' = "hello"
/// [3] 'table' = 0x213542
/// [4] 'function' = 0x138252
/// [5] 'nil' = nil
/// ```
pub fn dump_stack(L: LuaState) {
	use crate::globals::Lua::Type::*;
	use crate::lua_shared::*;

	let top = lua_gettop(L);
	for i in 1..=top {
		print!("[{}] '{}' = ", i, rstr!(luaL_typename(L, i)));
		match lua_type(L, i) {
			Number => println!("{}", lua_tonumber(L, i)),
			String => println!("{}", rstr!(lua_tostring(L, i))),
			Bool => println!(
				"{}",
				if lua_toboolean(L, i) == 1 {
					"true"
				} else {
					"false"
				}
			),
			Nil => println!("nil"),
			None => println!("none"),
			_ => println!("{:p}", lua_topointer(L, i)),
		}
	}
}
