use super::{prelude::*, NetMessage, NetEnum};

#[vtable]
pub struct NetChannelHandler {
	#[offset(1)] // Ignore virtual constructor
	/// Called on network established
	pub ConnectionStart: extern "C" fn(chan: *mut NetChannel),

	/// Closed intentionally
	pub ConnectionClosing: extern "C" fn(reason: *const c_char),

	/// Error
	pub ConnectionCrashed: extern "C" fn(reason: *const c_char),

	/// Called each time a packet is received
	pub PacketStart: extern "C" fn(incoming_seq: c_int, outgoing_ack: c_int),

	pub PacketEnd: extern "C" fn(),

	/// Target is requesting a file
	pub FileRequested: extern "C" fn(filename: *const c_char, transfer_id: c_uint),

	/// File received from target
	pub FileReceived: extern "C" fn(filename: *const c_char, transfer_id: c_uint),

	/// File request denied by target
	pub FileDenied: extern "C" fn(filename: *const c_char, transfer_id: c_uint),

	/// File sent to target acknowledged
	pub FileSent: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	pub ShouldAcceptFile: extern "C" fn(filename: *const c_char, transfer_id: c_uint) -> bool,
}

#[vtable]
pub struct NetChannel {
	info: *mut *mut NetChannelInfo,
	#[skip(27)] // Set to position 26, pointer after NetChannelInfo
	pub SetDataRate: extern "C" fn(rate: c_float),
	pub RegisterMessage: extern "C" fn(msg: *mut NetMessage) -> bool,
	pub StartStreaming: extern "C" fn(challenge_nr: c_uint) -> bool,
	pub ResetStreaming: extern "C" fn(),
	pub SetTimeout: extern "C" fn(seconds: c_float),
	pub SetDemoRecorder: extern "C" fn(recorder: *mut c_void),
	pub SetChallengeNr: extern "C" fn(challenge_nr: c_uint),
	pub Reset: extern "C" fn(),
	pub Clear: extern "C" fn(),
	#[check(36)]
	pub Shutdown: extern "C" fn(reason: *const c_char),
	pub ProcessPlayback: extern "C" fn(),
	pub ProcessStream: extern "C" fn() -> bool,
	pub ProcessPacket: extern "C" fn(packet: *mut c_void, has_header: bool) -> bool,
	pub SendNetMsg: extern "C" fn(msg: &NetMessage, force_reliable: bool, voice: bool),
	#[skip(1)]
	pub SendFile: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	pub DenyFile: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	#[deprecated]
	pub RequestFile_Old: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	pub SetChoked: extern "C" fn(),
	pub SendDatagram: extern "C" fn(data: *mut c_void),
	// #[check(49)]
	pub Transmit: extern "C" fn(only_reliable: bool) -> bool,
	#[skip(1)]
	pub GetMsgHandler: extern "C" fn() -> *mut NetChannelHandler,
	pub GetDropNumber: extern "C" fn() -> c_int,
	pub GetSocket: extern "C" fn() -> c_int,
	pub GetChallengeNr: extern "C" fn() -> c_uint,

	// TODO: Rest of the functions
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
	#[check(25)]
	pub GetTimeoutSeconds: extern "C" fn() -> c_float
}

#[vtable]
pub struct CNetChan {
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
	#[check(25)]
	pub GetTimeoutSeconds: extern "C" fn() -> c_float,

	#[skip(1)] // Skip deconstructor
	pub SetDataRate: extern "C" fn(rate: c_float),
	pub RegisterMessage: extern "C" fn(msg: *mut NetMessage) -> bool,
	pub StartStreaming: extern "C" fn(challenge_nr: c_uint) -> bool,
	pub ResetStreaming: extern "C" fn(),
	pub SetTimeout: extern "C" fn(seconds: c_float),
	pub SetDemoRecorder: extern "C" fn(recorder: *mut c_void),
	pub SetChallengeNr: extern "C" fn(challenge_nr: c_uint),
	pub Reset: extern "C" fn(),
	pub Clear: extern "C" fn(),
	#[check(36)]
	pub Shutdown: extern "C" fn(reason: *const c_char),
	pub ProcessPlayback: extern "C" fn(),
	pub ProcessStream: extern "C" fn() -> bool,
	pub ProcessPacket: extern "C" fn(packet: *mut c_void, has_header: bool) -> bool,
	pub SendNetMsg: extern "C" fn(msg: &NetMessage, force_reliable: bool, voice: bool),
	#[skip(1)]
	pub SendFile: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	pub DenyFile: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	#[deprecated]
	pub RequestFile_Old: extern "C" fn(filename: *const c_char, transfer_id: c_uint),
	pub SetChoked: extern "C" fn(),
	pub SendDatagram: extern "C" fn(data: *mut c_void),
	// #[check(49)]
	pub Transmit: extern "C" fn(only_reliable: bool) -> bool,
	#[skip(1)]
	pub GetMsgHandler: extern "C" fn() -> *mut NetChannelHandler,
	pub GetDropNumber: extern "C" fn() -> c_int,
	pub GetSocket: extern "C" fn() -> c_int,
	pub GetChallengeNr: extern "C" fn() -> c_uint,
}