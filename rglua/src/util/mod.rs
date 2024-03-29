#![allow(unused)]

/// Creates *const i8 from a &str
/// This either takes a literal and appends a null char (\0) to it.
/// or if it is an expression, tries to make a CString from it.
/// Will panic if passed an expression that a CString could not be created from.
/// # Examples
/// ```rust
/// use rglua::prelude::*;
/// let a = b"Hello world!".as_ptr() as *const i8;
/// let b = cstr!("Hello world!");
/// unsafe { assert_eq!(*a, *b) };
///
/// let c = "Hello world!";
/// let d = cstr!(c); // Macro doesn't know this is a literal, so it will try to make a CString
/// unsafe { assert_eq!(*b, *d.as_ptr()) };
/// ```
#[macro_export]
macro_rules! cstr {
	($rstring:literal) => {
		concat!($rstring, "\0").as_ptr() as *const i8
	};
	($rstring:expr) => {
		std::ffi::CString::new($rstring).expect("Couldn't make CString from rust string")
	};
}

/// Tries to create a *const i8 from a &str
/// This either takes a literal and appends a null char (\0) to it.
/// or if it is a value, makes a cstring and returns the pointer to it.
/// # Examples
/// ```rust, should_panic
/// use rglua::prelude::*;
/// let a = b"Hello world!".as_ptr() as *const i8;
/// let b = try_cstr!("Hello world!");
/// unsafe { assert_eq!(*a, *b) } ;
///
/// let c = "Invalid! 👎 \0"; // Cannot have nulls inside of it.
/// let d = try_cstr!(c).unwrap();
/// ```
#[macro_export]
macro_rules! try_cstr {
	($rstring:literal) => {
		concat!($rstring, "\0").as_ptr() as *const i8
	};
	($rstring:expr) => {{
		std::ffi::CString::new($rstring)
	}};
}

/// Tries to convert a const char* to a &str
/// Will panic if the const char* is not valid utf-8
/// # Examples
/// ```rust
/// use rglua::prelude::*;
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
/// use rglua::prelude::*;
/// let cstr = cstr!("Hello World");
/// let rstr = try_rstr!(cstr);
/// assert!(rstr.is_ok()); // Should be perfectly valid to convert to utf8
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
/// use rglua::prelude::*;
/// fn gmod13_open(l: LuaState) {
///     let world = "world";
///     printgm!(l, "Hello {}!", world);
/// }
/// ```
macro_rules! printgm {
	($state:expr, $($x:expr),*) => {
		{
			let printargs = format!( $($x,)* );
			if let Ok(fmt) = std::ffi::CString::new(printargs) {
				$crate::lua::lua_getglobal( $state, $crate::cstr!("print") );
				$crate::lua::lua_pushstring( $state, fmt.as_ptr() );
				$crate::lua::lua_call( $state, 1, 0 );
			}
		}
	};
}

/// Creates an array of LuaRegs for you to be used with luaL_register
/// # Examples
/// Basic usage
/// ```rust
/// use rglua::prelude::*;
/// extern "C" fn max(l: LuaState) -> i32 { 0 }
/// extern "C" fn min(l: LuaState) -> i32 { 0 }
/// let my_library = reg! [
///     "max" => max,
///     "min" => min
/// ];
/// assert_eq!(my_library.len(), 3); // 2 functions + 1 internal null terminator
/// unsafe { assert_eq!(my_library[0].name, cstr!("max")) }; // Internally this is turned into &[ LuaReg { name: cstr!("max"), func: max }, ... ];
/// ```
/// Returns a &[crate::types::LuaReg]
#[macro_export]
macro_rules! reg {
	( $( $name:expr => $func:expr ),* ) => {
		&[ $( $crate::types::LuaReg { name: $crate::cstr!($name), func: Some($func) } ),*, $crate::types::LuaReg { name: std::ptr::null(), func: None } ]
	};
}

// get_from_interface using literal c strings.
#[cfg(feature = "interfaces")]
fn get_from_interface(
	iface: &str,
	factory: crate::interface::CreateInterfaceFn
) -> Result<*mut (), crate::interface::Error> {
	let mut status = 0;

	let iface = try_cstr!(iface)?;
	let result = factory(iface.as_ptr(), &mut status);

	if status == 0 && !result.is_null() {
		Ok(result as *mut ())
	} else {
		Err(crate::interface::Error::FactoryNotFound(
			iface.to_string_lossy().to_string()
		))
	}
}

/// Quickly retrieves access to a source engine interface for you.
/// You can either use it through iface!(file, name, typename) or iface!(name).
/// # Examples
/// ```rust
/// use rglua::prelude::*;
/// use rglua::interface::{EngineClient, self};
/// #[gmod_open]
/// fn entry(l: LuaState) -> Result<i32, interface::Error>  {
///     let engine: &mut EngineClient = iface!("engine", "VEngineClient015", EngineClient)?;
///     println!("Am I in game? {}", engine.IsInGame());
///     Ok(0)
/// }
/// ```
/// ```rust
/// use rglua::prelude::*;
/// use rglua::interface;
/// #[gmod_open]
/// fn entry(l: LuaState) -> Result<i32, interface::Error>  {
///    let engine: &mut interface::EngineClient = iface!(EngineClient)?;
///    println!("Am I in game? {}", engine.IsInGame());
///    Ok(0)
/// }
/// ```
#[macro_export]
macro_rules! iface {
	( LuaShared ) => {
		iface!("lua_shared", "LUASHARED003", $crate::interface::LuaShared)
	};
	( EngineClient ) => {
		iface!(
			"engine",
			"VEngineClient015",
			$crate::interface::EngineClient
		)
	};
	( EngineServer ) => {
		iface!(
			"engine",
			"VEngineServer021",
			$crate::interface::EngineServer
		)
	};
	( MdlCache ) => {
		iface!("datacache", "MDLCache004", $crate::interface::MdlCache)
	};
	( MaterialSystem ) => {
		iface!(
			"materialsystem",
			"VMaterialSystem080",
			$crate::interface::MaterialSystem
		)
	};
	( Panel ) => {
		iface!("vgui2", "VGUI_Panel009", $crate::interface::Panel)
	};
	( ConVar ) => {
		iface!("vstdlib", "VEngineCvar007", $crate::interface::ConVar)
	};

	( $name:literal, $iface:literal, $ty:ty ) => {{
		// Would use map and flatten but flatten is unstable. =(
		match unsafe { $crate::interface::get_interface_handle($name) } {
			Ok(handle) => {
				let mut status = 0;

				// Don't need to try_cstr since this is a literal.
				let result = handle(cstr!($iface), &mut status);

				if status == 0 && !result.is_null() {
					let ptr = result as *mut $ty;
					unsafe { ptr.as_mut() }
						.ok_or($crate::interface::Error::IFaceMut(String::from($iface)))
				} else {
					Err($crate::interface::Error::CreateInterface(
						status,
						String::from($iface),
					))
				}
			}
			Err(why) => Err($crate::interface::Error::Libloading(why)),
		}
	}};
}

use crate::types::LuaState;
/// Returns the current state of the lua stack without affecting it.
/// Comes out in this format:
/// ```text
/// [1] 'number' = 5000
/// [2] 'string' = "hello"
/// [3] 'table' = 0x213542
/// [4] 'function' = 0x138252
/// [5] 'nil' = nil
/// ```
pub fn dump_stack(l: LuaState) -> Result<String, std::fmt::Error> {
	use std::fmt::Write;

	use crate::lua::*;

	let mut buf = String::new();

	let top = lua_gettop(l);
	for i in 1..=top {
		write!(&mut buf, "[{}] '{}' = ", i, rstr!(luaL_typename(l, i)));
		match lua_type(l, i) {
			TNUMBER => write!(&mut buf, "{}", lua_tonumber(l, i)),
			TSTRING => write!(&mut buf, "{}", rstr!(lua_tostring(l, i))),
			TBOOLEAN => write!(
				&mut buf,
				"{}",
				if lua_toboolean(l, i) == 1 {
					"true"
				} else {
					"false"
				}
			),
			TNIL => write!(&mut buf, "nil"),
			TNONE => write!(&mut buf, "none"),
			TUSERDATA | TLIGHTUSERDATA => write!(&mut buf, "{:p}", lua_touserdata(l, i)),
			TTHREAD => write!(&mut buf, "{:p}", lua_tothread(l, i)),
			_ => write!(&mut buf, "Unknown type")
		}?
	}

	Ok(buf)
}
