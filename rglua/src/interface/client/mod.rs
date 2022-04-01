use super::{prelude::*, net::{NetChannelHandler, NetChannel, NetMessage}};
#[vtable]
/// Client retrieved from Server interface.
pub struct Client {
	base: *mut *mut NetChannelHandler,

	#[skip(1)] // Skip destructor
	Connect: extern "C" fn(name: *const c_char, userid: c_int, chan: *mut NetChannel, fakeply: bool, challenge: c_int) -> bool,
	Inactivate: extern "C" fn(),
	Reconnect: extern "C" fn(),

	// Variadics don't work so..
	Disconnect: extern "C" fn(reason: *const c_char, ...),
	GetPlayerSlot: extern "C" fn() -> c_int,
	GetUserID: extern "C" fn() -> c_int,
	// GetNetworkID: extern "C" fn() -> UserID,

	#[skip(1)]
	GetClientName: extern "C" fn() -> *const c_char,
	GetNetChannel: extern "C" fn() -> *mut NetChannel,
	// GetServer: extern "C" fn() -> *mut Server,

	#[skip(1)]
	GetUserSetting: extern "C" fn(cvar: *const c_char) -> *const c_char,
	GetNetworkIDString: extern "C" fn() -> *const c_char,

	SetRate: extern "C" fn(rate: c_int, force: bool),
	GetRate: extern "C" fn() -> c_int,

	SetUpdateRate: extern "C" fn(rate: c_int, force: bool),
	GetUpdateRate: extern "C" fn() -> c_int,

	Clear: extern "C" fn(),
	GetMaxAckTickCount: extern "C" fn() -> c_int,
	ExecuteStringCommand: extern "C" fn(command: *const c_char) -> bool,
	SendNetMsg: extern "C" fn(msg: &mut NetMessage, force_reliable: bool),
	ClientPrint: extern "C" fn(msg: *const c_char, ...),
	IsConnected: extern "C" fn() -> bool,
	IsSpawned: extern "C" fn() -> bool,
	IsActive: extern "C" fn() -> bool,
	IsFakeClient: extern "C" fn() -> bool,
	IsHLTV: extern "C" fn() -> bool,

	// IsReplay ?

	IsHearingClient: extern "C" fn(id: c_int) -> bool,
	IsProximityHearingClient: extern "C" fn(id: c_int) -> bool,
	SetMaxRoutablePayloadSize: extern "C" fn(size: c_int),
}