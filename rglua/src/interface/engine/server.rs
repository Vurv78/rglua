use super::prelude::*;
use crate::interface::net::NetChannelInfo;

use std::os::raw::c_char;

// Temp
pub type EDict = c_void;

#[repr(transparent)]
// Temp
pub struct CSteamID(u64);

#[vtable]
/// "VEngineServer021"
/// "engine"
pub struct EngineServer {
	pub ChangeLevel: extern "C" fn(s1: *const c_char, s2: *const c_char),
	pub IsMapValid: extern "C" fn(filename: *const c_char) -> c_int,
	pub IsDedicatedServer: extern "C" fn() -> bool,
	pub IsInEditMode: extern "C" fn() -> c_int,
	pub PrecacheModel: extern "C" fn(s: *const c_char, preload: bool) -> c_int,
	pub PrecacheSentenceFile: extern "C" fn(s: *const c_char, preload: bool) -> c_int,
	pub PrecacheDecal: extern "C" fn(name: *const c_char, preload: bool) -> c_int,
	pub PrecacheGeneric: extern "C" fn(s: *const c_char, preload: bool) -> c_int,
	pub IsModelPrecached: extern "C" fn(s: *const c_char) -> c_int,
	pub IsDecalPrecached: extern "C" fn(name: *const c_char) -> c_int,
	pub IsGenericPrecached: extern "C" fn(s: *const c_char) -> c_int,
	pub GetClusterForOrigin: extern "C" fn(origin: &Vector) -> c_int,
	pub GetPVSForCluster:
		extern "C" fn(cluster: c_int, outputpvslength: c_int, outputpvs: *mut u8) -> c_int,
	pub CheckOriginInPVS:
		extern "C" fn(origin: &Vector, checkpvs: *const u8, checkpvssize: c_int) -> bool,
	pub CheckBoxInPVS: extern "C" fn(
		mins: &Vector,
		maxs: &Vector,
		checkpvs: *const u8,
		checkpvssize: c_int
	) -> bool,
	pub GetPlayerUserId: extern "C" fn(ent: *const EDict) -> c_int,
	pub GetPlayerNetworkIDString: extern "C" fn(ent: *const EDict) -> *const c_char,
	pub GetEntityCount: extern "C" fn() -> c_int,
	pub IndexOfEDict: extern "C" fn(pEdict: *const EDict) -> c_int,
	// Given an entity index, returns the corresponding edict pointer
	pub PEntityOfEntIndex: extern "C" fn(iEntIndex: c_int) -> *mut EDict,
	pub GetPlayerNetInfo: extern "C" fn(playerIndex: c_int) -> *mut NetChannelInfo,
	// Allocate space for string and return index/offset of string in global string list
	// If iForceEdictIndex is not -1, then it will return the edict with that index. If that edict index
	// is already used, it'll return null.
	pub CreateEDict: extern "C" fn(iForceEdictIndex: c_int) -> *mut EDict,
	pub RemoveEDict: extern "C" fn(edict: *mut EDict),
	pub PvAllocEntPrivateData: extern "C" fn(u: c_long) -> *mut c_void,

	#[offset(36)]
	// Issue a command to the command parser as if it was typed at the server console
	pub ServerCommand: extern "C" fn(cmd: *const c_char),
	// Execute any commands currently in the command parser immediately (instead of once per frame)
	pub ServerExecute: extern "C" fn(),

	#[offset(39)]
	// Set the lightstyle to the specified value and network the change to any connected clients.
	// Note that val must not change place in memory for anything that's not compiled into your mod.
	pub LightStyle: extern "C" fn(style: c_int, val: &'static c_void),
	pub StaticDecal: extern "C" fn(
		origin: &Vector,
		decalIndex: c_int,
		entityIndex: c_int,
		modelIndex: c_int,
		lowpriority: bool
	),

	#[offset(45)]
	// Print a message to the client's console
	pub ClientPrintf: extern "C" fn(pEdict: *const EDict, msg: *const c_char),

	#[offset(49)]
	pub Time: extern "C" fn() -> c_float,
	// Set the client's crosshair angle
	pub CrosshairAngle: extern "C" fn(pEdict: *const EDict, pitch: c_float, yaw: c_float),
	pub GetGameDir: extern "C" fn(szGetGameDir: *mut c_char, maxlen: c_int),

	#[offset(53)]
	// Locks/unlocks the network string tables (.e.g, when adding bots to server, this needs to happen).
	// Be sure to reset the lock after executing your code!!!
	pub LockNetworkStringTables: extern "C" fn(lock: bool),

	// Create a bot with the given name.  Returns NULL if fake client can't be created
	pub CreateFakeClient: extern "C" fn(netname: *const c_char) -> *mut EDict,

	#[offset(55)]
	pub GetClientConVarValue:
		extern "C" fn(clientIndex: c_int, name: *const c_char) -> *const c_char,

	#[offset(72)]
	// Logs a message to the server log file
	pub LogPrint: extern "C" fn(msg: *const c_char),

	#[offset(80)]
	pub IsPaused: extern "C" fn() -> bool,
	// Sets a USERINFO convar for a fake client (bot)
	pub SetFakeClientConVarValue:
		extern "C" fn(pEdict: *mut EDict, cvar: *const c_char, value: *const c_char),

	#[offset(83)]
	pub IsInCommentaryMode: extern "C" fn() -> bool,

	#[offset(89)]
	pub IsInternalBuild: extern "C" fn() -> bool,

	#[offset(93)]
	pub MultiplayerEndGame: extern "C" fn(),
	pub ChangeTeam: extern "C" fn(teamName: *const c_char),

	#[offset(98)]
	pub GetAppID: extern "C" fn() -> c_int,
	pub IsLowViolence: extern "C" fn() -> bool,

	#[offset(101)]
	pub InsertServerCommand: extern "C" fn(str: *const c_char),
	// Fill in the player info structure for the specified player index (name, model, etc.)
	pub GetPlayerInfo: extern "C" fn(ent_num: c_int, pInfo: *mut PlayerInfo) -> bool,
	pub IsClientFullyAuthenticated: extern "C" fn(pEdict: *const EDict) -> bool,

	#[offset(107)]
	pub GetClientSteamID: extern "C" fn(pEdict: *const EDict) -> *const CSteamID,

	#[offset(108)]
	pub GetServerSteamID: extern "C" fn() -> *const CSteamID,

	#[offset(110)]
	pub GetClientSteamIDByPlayerIndex: extern "C" fn(playerIndex: c_int) -> *const CSteamID,

	#[offset(113)]
	pub CreateFakeClientEx:
		extern "C" fn(netname: *const c_char, reportFakeClient: bool) -> *mut EDict,
	pub GetServerVersion: extern "C" fn() -> c_int,
	pub GMOD_SetTimeManipulator: extern "C" fn(fScaleFramerate: c_float) -> c_float,

	#[offset(117)]
	pub GMOD_SendToClient: extern "C" fn(client: c_int, msg: *const c_char, len: c_int),
	pub GMOD_RawServerCommand: extern "C" fn(cmd: *const c_char)
}
