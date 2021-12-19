use super::materials::IMaterial;
use super::prelude::*;

use std::os::raw::c_char;

iface! {
	/// VEngineClient015
	/// Offsets are confirmed correct as of 12/19/2021
	pub abstract struct EngineClient {};
}

impl EngineClient {
	#[virtual_index(2)]
	#[cfg(feature = "userdata")]
	pub fn TraceLineMaterialAndLighting(
		&self,
		start: &Vector,
		end: &Vector,
		diffuseLightColor: &mut Vector,
		baseColor: &mut Vector,
	) -> *const IMaterial {
	}

	#[virtual_index(5)]
	pub fn GetScreenSize(&self, width: *mut c_int, height: *mut c_int) {}

	#[virtual_index(6)]
	pub fn ServerCmd(&self, szCmdString: *const c_char, bReliable: bool) {}

	#[virtual_index(7)]
	pub fn ClientCmd(&self, szCmdString: *const c_char) {}

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

	#[virtual_index(35)]
	pub fn GetGameDirectory(&self) -> *const c_char {}

	#[virtual_index(51)]
	pub fn GetLevelName(&self) -> *const c_char {}

	#[virtual_index(76)]
	pub fn IsPlayingDemo(&self) -> bool {}

	#[virtual_index(77)]
	pub fn IsRecordingDemo(&self) -> bool {}

	#[virtual_index(84)]
	pub fn IsPaused(&self) -> bool {}

	#[virtual_index(102)]
	/// This is NOT checked against the FCVAR_CLIENTCMD_CAN_EXECUTE vars
	pub fn ExecuteClientCmd(&self, command: *const c_char) {}

	#[virtual_index(106)]
	/// This is NOT checked against the FCVAR_CLIENTCMD_CAN_EXECUTE vars
	pub fn ClientCmd_Unrestricted(&self, command: *const c_char) {}
}
