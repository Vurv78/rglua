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
