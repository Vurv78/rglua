use crate::interface::{MaterialSystem, NetChannelInfo};

use super::common::ButtonCode;
use super::materials::Material;
use super::prelude::*;

use std::os::raw::c_char;
use viable::vtable;

#[repr(C)]
pub struct ClientTextMessage {
	pub effect: c_int,
	pub col1: [u8; 4], // rgba
	pub col2: [u8; 4], // rgba
	pub x: c_float,
	pub y: c_float,
	pub fadein: c_float,
	pub fadeout: c_float,
	pub holdtime: c_float,
	pub fxtime: c_float,
	/// May be null. In that case it is default font.
	pub scheme_font_name: *const c_char,
	pub ply_name: *const c_char,
	pub msg: *const c_char,
	pub backdrop_box: bool,
	pub fl_box_size: c_float,
	pub box_color: [u8; 4], // rgba

	// char const*
	pub clear_msg: *mut c_char
}

#[vtable]
pub struct EngineClient {
	#[offset(1)]
	#[cfg(feature = "userdata")]
	pub GetLightForPoint: extern "C" fn(pos: &Vector, bClamp: bool) -> Vector,

	#[cfg(feature = "userdata")]
	pub TraceLineMaterialAndLighting: extern "C" fn(
		start: &Vector,
		end: &Vector,
		diffuseLightColor: &mut Vector,
		baseColor: &mut Vector
	) -> *const Material,

	#[offset(5)]
	pub GetScreenSize: extern "C" fn(width: &mut c_int, height: &mut c_int),
	pub ServerCmd: extern "C" fn(szCmdString: *const c_char, bReliable: bool),
	pub ClientCmd: extern "C" fn(szCmdString: *const c_char),
	pub GetPlayerInfo: extern "C" fn(player_index: c_int, info: &mut PlayerInfo) -> bool,
	pub GetPlayerForUserID: extern "C" fn(userID: c_int) -> c_int,
	pub TextMessageGet: extern "C" fn(pName: *const c_char) -> *mut ClientTextMessage,
	pub IsConsoleVisible: extern "C" fn() -> bool,
	pub GetLocalPlayer: extern "C" fn() -> c_int,

	#[offset(14)]
	pub Time: extern "C" fn() -> c_float,
	pub GetLastTimeStamp: extern "C" fn() -> c_float,

	#[offset(21)]
	pub GetMaxClients: extern "C" fn() -> c_int,
	/// Given the string pBinding which may be bound to a key, returns the name of the key to which this string is bound.
	/// Returns nullptr if no such binding exists
	pub Key_LookupBinding: extern "C" fn(binding: *const c_char) -> *const c_char,
	/// Given the string pBinding which may be bound to a key, returns the name of the key to which this string is bound.
	/// Returns nullptr if no such binding exists
	pub Key_BindingForKey: extern "C" fn(code: ButtonCode) -> *const c_char,
	pub StartKeyTrapMode: extern "C" fn(),
	pub CheckDoneKeyTrapping: extern "C" fn(code: &mut ButtonCode) -> bool,
	pub IsInGame: extern "C" fn() -> bool,
	pub IsConnected: extern "C" fn() -> bool,
	pub IsDrawingLoadingImage: extern "C" fn() -> bool,

	#[offset(35)]
	pub GetGameDirectory: extern "C" fn() -> *const c_char,

	#[offset(38)]
	pub GameLumpVersion: extern "C" fn(lump_id: c_int) -> c_int,
	pub GameLumpSize: extern "C" fn(lump_id: c_int) -> c_int,
	pub LoadGameLump: extern "C" fn(lump_id: c_int, pBuffer: *mut c_void, size: c_int) -> bool,
	pub LevelLeafCount: extern "C" fn() -> c_int,
	pub GetBSPTreeQuery: extern "C" fn() -> *mut c_void,
	pub LinearToGamma: extern "C" fn(linear: c_float, gamma: c_float),
	pub LightStyleValue: extern "C" fn(style: c_int) -> c_float,
	pub ComputeDynamicLighting: extern "C" fn(pt: &Vector, normal: &Vector, color: &mut Vector),
	pub GetAmbientLightColor: extern "C" fn(col: &mut Vector),
	pub GetDXSupportLevel: extern "C" fn() -> c_int,
	pub SupportsHDR: extern "C" fn() -> bool,
	/// Replace current material system pointer
	pub Mat_Stub: extern "C" fn(pMatSys: *mut MaterialSystem),
	pub GetChapterName: extern "C" fn(pchBuff: *mut c_char, maxlen: c_int),
	pub GetLevelName: extern "C" fn() -> *const c_char,
	pub GetLevelVersion: extern "C" fn() -> c_int,

	// Might need to look into this
	pub GetVoiceTweakApi: extern "C" fn() -> *mut c_void,
	pub EngineStats_BeginFrame: extern "C" fn(),
	pub EngineStats_EndFrame: extern "C" fn(),
	pub FireEvents: extern "C" fn(),
	pub GetLeavesArea: extern "C" fn(pLeaves: *mut c_int, nLeaves: c_int) -> c_int,
	pub DoesBoxTouchAreaFrustum: extern "C" fn(mins: &Vector, maxs: &Vector, iarea: c_int) -> bool, // 58

	#[offset(67)]
	pub ComputeLighting: extern "C" fn(
		pt: &Vector,
		pNormal: &Vector,
		clamp: bool,
		pColor: &mut Vector,
		pBoxColors: *mut Vector
	),
	pub ActivateOccluder: extern "C" fn(iOccluderIndex: c_int, bActive: bool),
	pub IsOccluded: extern "C" fn(vecAbsMins: &Vector, vecAbsMaxs: &Vector) -> bool,
	pub SaveAllocMemory: extern "C" fn(num: usize, size: usize) -> *mut c_void,
	pub SaveFreeMemory: extern "C" fn(pSaveMem: *mut c_void),

	#[offset(72)]
	pub GetNetChannelInfo: extern "C" fn() -> *mut NetChannelInfo,

	#[offset(76)]
	pub IsPlayingDemo: extern "C" fn() -> bool,
	pub IsRecordingDemo: extern "C" fn() -> bool,

	#[offset(84)]
	pub IsPaused: extern "C" fn() -> bool,
	pub IsTakingScreenshot: extern "C" fn() -> bool,
	pub IsHLTV: extern "C" fn() -> bool,
	pub IsLevelMainMenuBackground: extern "C" fn() -> bool,
	pub GetMainMenuBackgroundName: extern "C" fn(dest: *mut c_char, destlen: c_int),

	#[offset(91)]
	pub GetUILanguage: extern "C" fn(dest: *mut c_char, destlen: c_int),

	#[offset(94)]
	pub IsInEditMode: extern "C" fn() -> bool,
	pub GetScreenAspectRatio: extern "C" fn() -> c_float,

	#[offset(101)]
	pub IsHammerRunning: extern "C" fn() -> bool,
	/// This is NOT checked against the FCVAR_CLIENTCMD_CAN_EXECUTE vars
	pub ExecuteClientCmd: extern "C" fn(cmd: *const c_char),
	pub MapHasHDRLighting: extern "C" fn() -> bool,
	pub GetAppID: extern "C" fn() -> c_int,

	#[cfg(feature = "userdata")]
	pub GetLightForPointFast: extern "C" fn(pos: &Vector, bClamp: bool) -> Vector,

	#[offset(106)]
	/// This is NOT checked against the FCVAR_CLIENTCMD_CAN_EXECUTE vars
	pub ClientCmd_Unrestricted: extern "C" fn(cmd: *const c_char),
	pub SetRestrictServerCommands: extern "C" fn(bRestrict: bool),
	pub SetRestrictClientCommands: extern "C" fn(bRestrict: bool),
	pub SetOverlayBindProxy: extern "C" fn(id: c_int, proxy: *mut c_void),
	pub CopyFrameBufferToMaterial: extern "C" fn(matname: *const c_char) -> bool,
	pub ChangeTeam: extern "C" fn(team: *const c_char),
	pub ReadConfiguration: extern "C" fn(readdef: bool),
	pub SetAchievementMgr: extern "C" fn(pAchievementMgr: *mut c_void),
	pub GetAchievementMgr: extern "C" fn() -> *mut c_void,
	pub MapLoadFailed: extern "C" fn() -> bool,
	pub SetMapLoadFailed: extern "C" fn(bState: bool),
	pub IsLowViolence: extern "C" fn() -> bool,
	pub GetMostRecentSaveGame: extern "C" fn() -> *const c_char,
	pub SetMostRecentSaveGame: extern "C" fn(lpszFilename: *const c_char)
}
