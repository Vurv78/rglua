use vtables::VTable;
use vtables_derive::*;

use super::structs::ILuaInterface;

#[has_vtable]
#[derive(VTable)]
pub struct CLuaShared {}

impl CLuaShared {
	#[virtual_index(6)]
	pub fn GetLuaInterface(&self, realm: u8) -> *mut ILuaInterface {}
}