use super::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub enum NetEnum {
	Generic = 0,
	LocalPlayer,
	OtherPlayers,
	Entities,
	Sounds,
	Events,
	Usermessages,
	EntMessages,
	Voice,
	Stringtable,
	Move,
	Stringcmd,
	Signon,
	Total
}

#[vtable]
pub struct NetChannelInfo {
	pub ty: NetEnum,

	pub GetName: extern "C" fn() -> *const c_char,
	pub GetAddress: extern "C" fn() -> *const c_char,
	pub GetTime: extern "C" fn() -> c_float,
	pub GetTimeConnected: extern "C" fn() -> c_float,
	pub GetBufferSize: extern "C" fn() -> c_int,
	pub GetDataRate: extern "C" fn() -> c_int,
	pub IsLoopback: extern "C" fn() -> bool,
	pub IsTimingOut: extern "C" fn() -> bool,
	pub IsPlayback: extern "C" fn() -> bool,
	pub GetLatency: extern "C" fn(flow: c_int) -> c_float,
	pub GetAvgLatency: extern "C" fn(flow: c_int) -> c_float,
	pub GetAvgLoss: extern "C" fn(flow: c_int) -> c_float,
	pub GetAvgChoke: extern "C" fn(flow: c_int) -> c_float,
	pub GetAvgData: extern "C" fn(flow: c_int) -> c_float,
	pub GetAvgPackets: extern "C" fn(flow: c_int) -> c_float,
	pub GetTotalData: extern "C" fn(flow: c_int) -> c_int,
	pub GetSequenceNr: extern "C" fn(flow: c_int) -> c_int,
	pub IsValidPacket: extern "C" fn(flow: c_int, frame_number: c_int) -> bool,
	pub GetPacketTime: extern "C" fn(flow: c_int, frame_number: c_int) -> c_float,
	pub GetPacketBytes: extern "C" fn(flow: c_int, frame_number: c_int) -> c_int,
	pub GetStreamProgress:
		extern "C" fn(flow: c_int, received: *mut c_int, total: *mut c_int) -> bool,
	pub GetTimeSinceLastReceived: extern "C" fn() -> c_float,
	pub GetCommandInterpolationAmount: extern "C" fn(flow: c_int, frame_number: c_int) -> c_float,
	pub GetPacketResponseLatency: extern "C" fn(
		flow: c_int,
		frame_number: c_int,
		latencyms: *mut c_int,
		pnchoke: *mut c_int
	) -> c_float,
	pub GetRemoteFramerate: extern "C" fn(
		pflFrameTime: *mut c_float,
		pflFrameTimeStdDeviation: *mut c_float
	) -> c_float,
	pub GetTimeoutSeconds: extern "C" fn() -> c_float
}
