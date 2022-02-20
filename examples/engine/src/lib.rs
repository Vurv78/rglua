use rglua::interface;
use rglua::prelude::*;

#[lua_function]
fn concmd(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;
	engine.ExecuteClientCmd( luaL_checkstring(l, 1) );
	Ok(0)
}

#[lua_function]
fn get_resolution(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	let (w, h) = (&mut 0, &mut 0);
	engine.GetScreenSize(w, h);

	lua_pushinteger(l, *w as isize);
	lua_pushinteger(l, *h as isize);

	Ok(2)
}

#[lua_function]
fn get_directory(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	let dir = engine.GetGameDirectory();
	lua_pushstring(l, dir);

	Ok(1)
}

#[lua_function]
fn get_level(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	let level = engine.GetLevelName();
	lua_pushstring(l, level);

	Ok(1)
}

#[lua_function]
fn is_recording(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	let demo = engine.IsRecordingDemo();
	lua_pushboolean(l, demo as i32);

	Ok(1)
}

#[lua_function]
fn is_paused(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	let paused = engine.IsPaused();
	lua_pushboolean(l, paused as i32);

	Ok(1)
}

#[lua_function]
fn is_console_visible(l: LuaState) -> Result<i32, interface::Error> {
	let engine = iface!(EngineClient)?;

	lua_pushboolean(l, engine.IsConsoleVisible() as i32);

	Ok(1)
}

#[lua_function]
fn net_infos(l: LuaState) -> Result<i32, interface::Error> {
	let flow = luaL_checkinteger(l, 1) as i32;

	let engine = iface!(EngineClient)?;

	let infos = unsafe { engine.GetNetChannelInfo().as_mut() }
		.ok_or(interface::Error::AsMut)?;

	lua_newtable(l);

	lua_pushstring(l, infos.GetName());
	lua_setfield(l, -2, cstr!("name"));

	lua_pushstring(l, infos.GetAddress());
	lua_setfield(l, -2, cstr!("address"));

	lua_pushnumber(l, infos.GetTime() as f64);
	lua_setfield(l, -2, cstr!("time"));

	lua_pushnumber(l, infos.GetTimeConnected() as f64);
	lua_setfield(l, -2, cstr!("time_connected"));

	lua_pushnumber(l, infos.GetBufferSize() as f64);
	lua_setfield(l, -2, cstr!("buffer_size"));

	lua_pushnumber(l, infos.GetDataRate() as f64);
	lua_setfield(l, -2, cstr!("data_rate"));

	lua_pushnumber(l, infos.GetAvgLoss(flow) as f64);
	lua_setfield(l, -2, cstr!("avg_loss"));

	lua_pushnumber(l, infos.GetAvgChoke(flow) as f64);
	lua_setfield(l, -2, cstr!("avg_choke"));

	lua_pushnumber(l, infos.GetAvgData(flow) as f64);
	lua_setfield(l, -2, cstr!("avg_data"));

	lua_pushnumber(l, infos.GetAvgLatency(flow) as f64);
	lua_setfield(l, -2, cstr!("avg_latency"));

	Ok(1)
}

#[gmod_open]
fn open(l: LuaState) -> Result<i32, interface::Error> {
	printgm!(l, "Loaded engine module!");

	let lua = iface!(LuaShared)?;

	let client = lua.GetLuaInterface(0);
	let menu = lua.GetLuaInterface(2);
	if client.is_null() && menu.is_null() {
		// TODO: Serverside support & Clientside in one module :)
		luaL_error(l, cstr!("This module is only compatible with the CLIENT or MENU states!"));
	}

	let lib = reg! [
		"concmd" => concmd,
		"getResolution" => get_resolution,
		"getGameDirectory" => get_directory,
		"getLevel" => get_level,
		"isRecording" => is_recording,
		"isPaused" => is_paused,
		"isConsoleVisible" => is_console_visible,
		"getNetInfo" => net_infos
	];

	luaL_register(l, cstr!("iengine"), lib.as_ptr());
	Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}
