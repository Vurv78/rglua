use super::prelude::*;
use super::{CLuaShared, ILuaInterface};

impl CLuaShared {
	#[virtual_index(4)]
	pub fn CreateLuaInterface(&self, c: c_uchar, b: bool) -> *mut ILuaInterface {}

	#[virtual_index(5)]
	pub fn CloseLuaInterface(&self, iface: *mut ILuaInterface) {}

	#[virtual_index(6)]
	pub fn GetLuaInterface(&self, realm: c_uchar) -> *mut ILuaInterface {}

	#[virtual_index(9)]
	pub fn MountLua(&self, l: *const c_char) {}

	#[virtual_index(10)]
	pub fn MountLuaAdd(&self, l: *const c_char, l2: *const c_char) {}

	#[virtual_index(11)]
	pub fn UnMountLua(&self, l: *const c_char) {}

	#[virtual_index(12)]
	pub fn SetFileContents(&self, l: *const c_char, c: *const c_char) {}
}
