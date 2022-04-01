use super::{prelude::*, NetChannel};

#[vtable]
pub struct NetMessage {
	#[skip(1)] // Skip destructor
	pub SetNetChannel: extern "C" fn(net_channel: *mut NetChannel),
	pub SetReliable: extern "C" fn(reliable: bool),
	pub Process: extern "C" fn() -> bool,
	pub ReadFromBuffer: extern "C" fn(buffer: &mut c_void) -> bool,
	pub WriteToBuffer: extern "C" fn(buffer: &mut c_void) -> bool,
	pub IsReliable: extern "C" fn() -> bool,
	pub GetType: extern "C" fn() -> c_int,
	pub GetGroup: extern "C" fn() -> c_int,
	pub GetName: extern "C" fn() -> *const c_char,
	pub GetNetChannel: extern "C" fn() -> *mut NetChannel,
	pub ToString: extern "C" fn() -> *const c_char,
}