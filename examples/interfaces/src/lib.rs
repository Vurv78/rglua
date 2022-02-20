use rglua::prelude::*;
use rglua::interface::{self, LuaShared};

#[derive(Debug, thiserror::Error)]
enum GenericError {
	#[error("Couldn't get interface {0}")]
	NoInterface(String),

	#[error("Couldn't get interface as reference {0}")]
	AsMut(String)
}

#[gmod_open]
fn open(l: LuaState) -> Result<i32, GenericError> {
	// Access the lua state when you aren't directly given it.
	let lua_shared = iface!(LuaShared)?;
	let menu = unsafe { lua_shared.GetLuaInterface(2).as_mut() }
		.ok_or(interface::Error::AsMut)?;

	printgm!(menu.base as _, "Hello from ILuaShared!");
	Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}