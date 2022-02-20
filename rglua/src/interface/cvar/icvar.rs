use super::prelude::*;
use super::CVar;

#[repr(C)]
#[derive(Debug)]
pub struct ConCommandBase {
	pub base_vtable: *mut c_void,   // 0
	pub next: *mut ConCommandBase,  // 4
	pub registered: bool,           // 8
	pub name: *const c_char,        // 12
	pub help_string: *const c_char, // 16
	pub flags: c_int                // 20
}

/// This is probably very wrong
/// "VEngineCvar007"
/// "vstdlib"
#[vtable]
pub struct ConVar {
	pub RegisterConCommand: extern "C" fn(pCommandBase: *mut ConCommandBase),
	pub UnregisterConCommand: extern "C" fn(pCommandBase: *mut ConCommandBase),
	#[offset(7)]
	pub FindVar: extern "C" fn(var_name: *const c_char) -> *mut CVar,

	#[offset(11)]
	pub GetCommands: extern "C" fn() -> *mut ConCommandBase
}
