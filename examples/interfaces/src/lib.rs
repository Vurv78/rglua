use rglua::prelude::*;
use rglua::interface::{self, NetChannel, CNetChan};

#[lua_function]
fn reload_textures(_l: LuaState) -> Result<i32, interface::Error> {
	let matsys = iface!(MaterialSystem)?;
	matsys.ReloadTextures();

	Ok(0)
}

#[lua_function]
fn disconnect(l: LuaState) -> Result<i32, interface::Error> {
	let msg = luaL_checkstring(l, 1);
	let engine = iface!(EngineClient)?;

	let chan = engine.GetNetChannelInfo() as *mut CNetChan;
	let chan = unsafe { chan.as_mut() }
		.ok_or(interface::Error::AsMut)?;

	printgm!(l, "{:?}", try_rstr!(chan.GetAddress()));

	chan.Clear();
	chan.Shutdown(msg);
	return Ok(0);
}

#[gmod_open]
fn open(l: LuaState) -> Result<i32, interface::Error> {
	// Access the lua state when you aren't directly given it.
	let lua_shared = iface!(LuaShared)?;

	// 0 to get client state. this will error if you try and run the binary module on the server or menu realms.
	let client = unsafe { lua_shared.GetLuaInterface(0).as_mut() }
		.ok_or(interface::Error::AsMut)?;

	printgm!(client.base as _, "Hello from ILuaShared!");

	let lib = reg! [
		"reloadTextures" => reload_textures,
		"disconnect" => disconnect
	];

	luaL_register(l, cstr!("interfaces"), lib.as_ptr());

	Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}