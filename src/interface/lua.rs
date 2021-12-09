use super::prelude::*;

#[has_vtable]
#[derive(VTable)]
pub struct ILuaInterface {
	pub vtable: usize,
}

impl ILuaInterface {
	#[virtual_index(20)]
	pub fn IsServer(&self) -> bool {}

	#[virtual_index(21)]
	pub fn IsClient(&self) -> bool {}

	#[virtual_index(22)]
	pub fn IsMenu(&self) -> bool {}
}

#[has_vtable]
#[derive(VTable)]
pub struct CLuaShared {}

impl CLuaShared {
	#[virtual_index(6)]
	pub fn GetLuaInterface(&self, realm: u8) -> *mut ILuaInterface {}
}
