
pub mod structs {
	use vtables::VTable;
	use vtables_derive::*;

	#[repr(C)]
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
		pad: [i8; 304]
	}

	#[has_vtable]
	#[derive(VTable)]
	pub struct ILuaInterface {
		pub vtable: usize
	}

	impl ILuaInterface {
		#[virtual_index(20)]
		pub fn IsServer(&self) -> bool {}

		#[virtual_index(21)]
		pub fn IsClient(&self) -> bool {}

		#[virtual_index(22)]
		pub fn IsMenu(&self) -> bool {}
	}
}

mod engine;
pub use engine::EngineClient;

mod lua;
pub use lua::CLuaShared;

mod panel;
pub use panel::IPanel;