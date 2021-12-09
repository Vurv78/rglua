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
pub struct LuaReg {
	pub name: *const i8, // c_schar
	pub func: Option<LuaCFunction>,
}

pub type LuaJITProfileCallback = extern "C" fn(data: *mut c_void, l: LuaState, samples: c_int, vmL: c_int) -> ();


#[repr(C)]
pub struct LuaBuffer {
	pub b: *mut i8,
	pub size: SizeT,
	pub n: SizeT, // number of chars in buffer
	pub state: LuaState,
	pub initbuffer: [i8; 8192_usize],
}
