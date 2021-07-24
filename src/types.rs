use std::os::raw as ffi;

// C FFI Types
pub type CVoid = ffi::c_void;
pub type CInt = ffi::c_int;
pub type CLong = ffi::c_long;
pub type CharBuf = *const ffi::c_char; //*const i8; // const char*
pub type SizeT = usize;

// Lua Types below
pub type LuaNumber = f64; // All lua numbers are doubles in Lua 5.1 (Glua)
pub type LuaInteger = isize;

pub type LuaState = *mut CVoid; // Raw Lua state.
pub type LuaCFunction = extern "C" fn(LuaState) -> CInt;

#[repr(C)]
pub struct LuaL_Buffer {
	b: *mut i8,
	size: SizeT,
	n: SizeT, // number of chars in buffer
	state: LuaState,
	initbuffer: [i8; 8192_usize],
}