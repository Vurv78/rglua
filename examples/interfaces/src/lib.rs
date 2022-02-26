use rglua::prelude::*;
use rglua::interface;

#[lua_function]
fn reload_textures(_l: LuaState) -> Result<i32, interface::Error> {
	let matsys = iface!(MaterialSystem)?;
	matsys.ReloadTextures();

	Ok(0)
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
		"reloadTextures" => reload_textures
	];

	luaL_register(l, cstr!("interfaces"), lib.as_ptr());

	Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}