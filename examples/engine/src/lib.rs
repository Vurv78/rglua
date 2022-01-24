use rglua::interface::{EngineClient};
use rglua::prelude::*;

fn get_iface() -> Option<&'static EngineClient> {
	let iface: *mut EngineClient = iface!("engine", "VEngineClient015")?;
	unsafe { iface.as_ref() }
}

#[lua_function]
fn concmd(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		iface.ExecuteClientCmd( luaL_checkstring(l, 1) );
	}
	0
}

#[lua_function]
fn get_resolution(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		let (w, h): (*mut _, *mut _) = (&mut 0, &mut 0);
		iface.GetScreenSize(w, h);
		unsafe {
			lua_pushinteger(l, *w as isize);
			lua_pushinteger(l, *h as isize);
		}
		return 2;
	}
	0
}

#[lua_function]
fn get_directory(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		let dir = iface.GetGameDirectory();
		lua_pushstring(l, dir);
		return 1;
	}
	0
}

#[lua_function]
fn get_level(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		let level = iface.GetLevelName();
		lua_pushstring(l, level);
		return 1;
	}
	0
}

#[lua_function]
fn is_recording(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		let demo = iface.IsRecordingDemo();
		lua_pushboolean(l, demo as i32);
		return 1;
	}
	0
}

#[lua_function]
fn is_paused(l: LuaState) -> i32 {
	if let Some(iface) = get_iface() {
		let paused = iface.IsPaused();
		lua_pushboolean(l, paused as i32);
		return 1;
	}
	0
}

#[gmod_open]
fn open(l: LuaState) -> i32 {
	printgm!(l, "Loaded engine module!");

	let lib = reg! [
		"concmd" => concmd,
		"getResolution" => get_resolution,
		"getGameDirectory" => get_directory,
		"getLevel" => get_level,
		"isRecording" => is_recording,
		"isPaused" => is_paused
	];

	luaL_register(l, cstr!("iengine"), lib.as_ptr());
	0
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}
