use super::prelude::*;

iface! {
	pub abstract struct IPanel {};
}

impl IPanel {
	#[virtual_index(36)]
	pub fn GetName(&self, vguiPanel: u32) -> *const i8 {}

	#[virtual_index(41)]
	/// PaintTraverse function, notorious for getting you banned from every other source engine game.
	/// Lua runs properly here, so maybe you'd want to detour this.
	pub fn PaintTraverse(&self, vguiPanel: u32, forceRepaint: bool, allowForce: bool) {}
}
