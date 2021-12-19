use std::os::raw as ffi;

// C FFI Types
pub use ffi::{c_int, c_long, c_void};
pub type LuaString = *const ffi::c_char; // const i8
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
	pub initbuffer: [i8; 8192_usize],
}
