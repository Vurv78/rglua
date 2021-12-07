use std::os::raw as ffi;

// C FFI Types
pub use ffi::{c_int, c_long, c_void};
pub type LuaString = *const ffi::c_char; // const i8
pub type SizeT = usize;

// Lua Types below
pub type LuaNumber = f64; // All lua numbers are doubles in Lua 5.1 (Glua)
pub type LuaInteger = isize;

pub type LuaState = *mut c_void; // Raw Lua state.
pub type LuaCFunction = extern "C" fn(LuaState) -> c_int;

#[repr(C)]
pub struct LuaL_Buffer {
	b: *mut i8,
	size: SizeT,
	n: SizeT, // number of chars in buffer
	state: LuaState,
	initbuffer: [i8; 8192_usize],
}
