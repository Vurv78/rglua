use std::os::raw as ffi;

// C FFI Types
pub use ffi::{c_char, c_int, c_long, c_void};

pub type LuaString = *const c_char; // const i8
pub type SizeT = usize;

// Lua Types below

/// All lua numbers < Lua 5.3 are doubles (float 64). GLua is no different as it runs on LuaJIT which mimics 5.1
pub type LuaNumber = f64;
pub type LuaInteger = isize;

/// This is not an actual lua state, in fact it's just a pointer to it.
/// However you will never have ownership of a lua state here, so I opted to make the type follow suit.
pub type LuaState = *mut c_void; // Raw Lua state.

/// Lua "C" Functions are C ABI functions that return the number returns that will be passed to the Lua stack.
pub type LuaCFunction = extern "C" fn(LuaState) -> c_int;
pub type LuaHook = extern "C" fn(LuaState, *mut LuaDebug) -> c_int;
pub type LuaAlloc =
	extern "C" fn(ud: *mut c_void, ptr: *mut c_void, osize: usize, nsize: usize) -> *mut c_void;

pub type LuaReader = extern "C" fn(LuaState, ud: *mut c_void, sz: *mut SizeT) -> *const u8;

/// luaL_Reg type, used for defining large amounts of functions with names to be -
/// registered into lua with luaL_register / openlibs.
#[repr(C)]
pub struct LuaReg {
	pub name: *const i8, // c_schar
	pub func: Option<LuaCFunction>,
}

pub type LuaJITProfileCallback =
	extern "C" fn(data: *mut c_void, l: LuaState, samples: c_int, vmL: c_int) -> ();

#[repr(C)]
pub struct LuaBuffer {
	pub b: *mut i8,
	pub size: SizeT,
	pub n: SizeT, // number of chars in buffer
	pub state: LuaState,
	pub initbuffer: [i8; crate::lua::BUFFERSIZE],
}

#[derive(Debug, Clone)]
#[repr(C)]
/// Lua's lua_Debug type
pub struct LuaDebug {
	/// (n)
	pub event: c_int,
	/// (n)
	pub name: LuaString, // (n)
	/// (n) - `global' | `local' | `field' | `method'
	pub namewhat: LuaString,
	/// (S) - `Lua' | `C' | `main' | `tail'
	pub what: LuaString,
	/// (S)
	pub source: LuaString,
	/// (l)
	pub currentline: c_int,
	/// (u)
	pub nups: c_int,
	/// (S)
	pub linedefined: c_int,
	/// (S)
	pub lastlinedefined: c_int,
	/// (S)
	pub short_src: [c_char; crate::lua::IDSIZE],

	/// This should be private but whatever.
	pub i_ci: c_int, /* active function */
}

impl Default for LuaDebug {
	fn default() -> Self {
		Self {
			name: std::ptr::null(),
			namewhat: std::ptr::null(),
			what: std::ptr::null(),
			source: std::ptr::null(),
			..Default::default()
		}
	}
}
