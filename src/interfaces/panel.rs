use vtables::VTable;
use vtables_derive::*;

#[has_vtable]
#[derive(VTable)]
pub struct IPanel {
	pub vtable: usize
}

impl IPanel {
	#[virtual_index(36)]
	pub fn GetName(&self, vguiPanel: u32) -> *const i8 {}

	#[virtual_index(41)]
	pub fn PaintTraverse(&self, vguiPanel: u32, forceRepaint: bool, allowForce: bool) {}
}