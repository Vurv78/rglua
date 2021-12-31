use super::prelude::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct PlayerInfo {
	unknown: u64,
	xuid: u64,
	name: [i8; 32],
	unknown01: [char; 96],
	m_szPlayerName: [char; 128],
	m_nUserID: i32,
	m_szSteamID: [char; 33],
	m_nSteam3ID: u32,
	userID: i32,
	guid: [char; 33],
	friendsID: i32,
	fakeplayer: bool,
	ishltv: bool,
	customFiles: [u64; 4],
	filesDownloaded: u8,
	pad: [i8; 304],
}

#[repr(C)]
pub struct StudioHdr {
	pub id: c_int,
	pub version: c_int,
	pub checksum: c_int,

	pub name: [c_char; 64],
	pub length: c_int,

	pub eyeposition: Vector,
	pub illumposition: Vector,
	pub hull_min: Vector,
	pub hull_max: Vector,
	pub view_bbmin: Vector,
	pub view_bbmax: Vector,
	pub flags: c_int,
	pub numbones: c_int,
	pub boneindex: c_int,
	pub numbonecontrollers: c_int,
	pub bonecontrollerindex: c_int,
	pub numhitboxsets: c_int,
	pub hitboxsetindex: c_int,
}
