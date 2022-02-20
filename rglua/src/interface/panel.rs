use super::prelude::*;
/// "VGUI_Panel009"
/// "vgui2"
#[vtable]
pub struct Panel {
	#[offset(36)]
	pub GetName: extern "C" fn(vguiPanel: u32) -> *const c_char,

	#[offset(41)]
	/// PaintTraverse function, notorious for getting you banned from every other source engine game.
	/// Lua runs properly here, so maybe you'd want to detour this.
	pub PaintTraverse: extern "C" fn(vguiPanel: u32, forceRepaint: bool, allowForce: bool)
}
