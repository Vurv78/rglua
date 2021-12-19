use super::prelude::*;

#[repr(C)]
pub struct ConCommandBase {
	pub base_vtable: *mut c_void,   // 0
	pub next: *mut ConCommandBase,  // 4
	pub registered: bool,           // 8
	pub name: *const c_char,        // 12
	pub help_string: *const c_char, // 16
	pub flags: c_int,               // 20
}

iface! {
	/// VEngineCvar007
	/// vstdlib.dll
	pub abstract struct ICVar {};
}

impl ICVar {
	#[virtual_index(1)]
	pub fn RegisterConCommand(&self, pCommandBase: *mut ConCommandBase) {}

	#[virtual_index(2)]
	pub fn UnregisterConCommand(&self, pCommandBase: *mut ConCommandBase) {}

	#[virtual_index(11)]
	pub fn GetCommands(&self) -> *mut ConCommandBase {}
}
