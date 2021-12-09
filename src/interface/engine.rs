use super::prelude::*;

use std::os::raw::c_char;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {
	pub vtable: usize,
}

impl EngineClient {
	#[virtual_index(5)]
	pub fn GetScreenSize(&self, width: *mut i32, height: *mut i32) {}

	#[virtual_index(8)]
	pub fn GetPlayerInfo(&self, p_info: *mut PlayerInfo) -> bool {}

	#[virtual_index(9)]
	pub fn GetPlayerForUserID(&self, userid: i32) -> i32 {}

	#[virtual_index(12)]
	pub fn GetLocalPlayer(&self) -> i32 {}

	#[virtual_index(14)]
	pub fn Time(&self) -> f32 {}

	#[virtual_index(26)]
	pub fn IsInGame(&self) -> bool {}

	#[virtual_index(27)]
	pub fn IsConnected(&self) -> bool {}

	#[virtual_index(108)]
	pub fn ExecuteClientCmd(&self, command: *const c_char) {}

	#[virtual_index(113)]
	pub fn ClientCmd_Unrestricted(&self, command: *const c_char) {}
}
