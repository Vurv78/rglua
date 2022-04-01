use super::prelude;

#[repr(C)]
#[derive(Debug)]
pub enum NetEnum {
	Generic = 0,
	LocalPlayer,
	OtherPlayers,
	Entities,
	Sounds,
	Events,
	UserMessages,
	EntMessages,
	Voice,
	StringTable,
	Move,
	StringCmd,
	SignOn,
	Total
}

mod message;
pub use message::NetMessage;
mod channel;
pub use channel::{NetChannelHandler, NetChannel, NetChannelInfo, CNetChan};