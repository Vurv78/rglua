use super::prelude::*;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
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
	pub hitboxsetindex: c_int
}

#[repr(C)]
pub enum ButtonCode {
	Invalid = -1,
	None = 0,

	Key0,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	KeyA,
	KeyB,
	KeyC,
	KeyD,
	KeyE,
	KeyF,
	KeyG,
	KeyH,
	KeyI,
	KeyJ,
	KeyK,
	KeyL,
	KeyM,
	KeyN,
	KeyO,
	KeyP,
	KeyQ,
	KeyR,
	KeyS,
	KeyT,
	KeyU,
	KeyV,
	KeyW,
	KeyX,
	KeyY,
	KeyZ,
	KeyPad0,
	KeyPad1,
	KeyPad2,
	KeyPad3,
	KeyPad4,
	KeyPad5,
	KeyPad6,
	KeyPad7,
	KeyPad8,
	KeyPad9,
	KeyPadDIVIDE,
	KeyPadMULTIPLY,
	KeyPadMINUS,
	KeyPadPLUS,
	KeyPadENTER,
	KeyPadDECIMAL,
	KeyLBRACKET,
	KeyRBRACKET,
	KeySEMICOLON,
	KeyAPOSTROPHE,
	KeyBACKQUOTE,
	KeyCOMMA,
	KeyPERIOD,
	KeySLASH,
	KeyBACKSLASH,
	KeyMINUS,
	KeyEQUAL,
	KeyENTER,
	KeySPACE,
	KeyBACKSPACE,
	KeyTAB,
	KeyCAPSLOCK,
	KeyNUMLOCK,
	KeyESCAPE,
	KeySCROLLLOCK,
	KeyINSERT,
	KeyDELETE,
	KeyHOME,
	KeyEND,
	KeyPAGEUP,
	KeyPAGEDOWN,
	KeyBREAK,
	KeyLSHIFT,
	KeyRSHIFT,
	KeyLALT,
	KeyRALT,
	KeyLCONTROL,
	KeyRCONTROL,
	KeyLWIN,
	KeyRWIN,
	KeyAPP,
	KeyUP,
	KeyLEFT,
	KeyDOWN,
	KeyRIGHT,
	KeyF1,
	KeyF2,
	KeyF3,
	KeyF4,
	KeyF5,
	KeyF6,
	KeyF7,
	KeyF8,
	KeyF9,
	KeyF10,
	KeyF11,
	KeyF12,
	KeyCAPSLOCKTOGGLE,
	KeyNUMLOCKTOGGLE,
	KeySCROLLLOCKTOGGLE,

	// Mouse
	MouseLeft,
	MouseRight,
	MouseMiddle,
	Mouse4,
	Mouse5,
	MouseWheelUp, // A fake button which is 'pressed' and 'released' when the wheel is moved up
	MouseWheelDown  // A fake button which is 'pressed' and 'released' when the wheel is moved down
}

#[allow(non_upper_case_globals)]
impl ButtonCode {
	pub const KeyFIRST: Self = Self::None;
	pub const KeyNONE: Self = Self::KeyFIRST;
	pub const KeyLAST: Self = Self::KeySCROLLLOCKTOGGLE;
	pub const KeyCOUNT: i32 = (Self::KeyLAST as i32 - Self::KeyFIRST as i32 + 1);
}

#[repr(C)]
pub enum SkyboxVisibility {
	NotVisible,
	Visible3D,
	Visible2D
}