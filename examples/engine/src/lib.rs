use rglua::interface::{get_from_interface, get_interface_handle, EngineClient};
use rglua::prelude::*;

use anyhow::bail;

fn get_iface() -> anyhow::Result<&'static EngineClient> {
	let handle = unsafe { get_interface_handle("engine.dll")? };
	let iface = get_from_interface("VEngineClient015", handle)? as *mut EngineClient;
	match unsafe { iface.as_ref() } {
		Some(iface) => Ok(iface),
		None => bail!("Failed to get interface"),
	}
}

#[lua_function]
fn concmd(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => {
			iface.ExecuteClientCmd( luaL_checklstring(l, 1, 0) );
		}
		Err(e) => printgm!(l, "{}", e)
	}
	0
}

#[lua_function]
fn get_resolution(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => unsafe {
			let (w, h): (*mut _, *mut _) = (&mut 0, &mut 0);
			iface.GetScreenSize(w, h);
			lua_pushinteger(l, *w as isize);
			lua_pushinteger(l, *h as isize);
			return 2;
		},
		Err(e) => {
			printgm!(l, "Failed to get interface: {}", e);
		}
	}
	0
}

#[lua_function]
fn get_directory(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => {
			let dir = iface.GetGameDirectory();
			lua_pushstring(l, dir);
			return 1;
		},
		Err(e) => {
			printgm!(l, "Failed to get interface: {}", e);
		}
	}
	0
}

#[lua_function]
fn get_level(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => {
			let level = iface.GetLevelName();
			lua_pushstring(l, level);
			return 1;
		},
		Err(e) => {
			printgm!(l, "Failed to get interface: {}", e);
		}
	}
	0
}

#[lua_function]
fn is_recording(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => {
			let demo = iface.IsRecordingDemo();
			lua_pushboolean(l, demo as i32);
			return 1;
		},
		Err(e) => {
			printgm!(l, "Failed to get interface: {}", e);
		}
	}
	0
}

#[lua_function]
fn is_paused(l: LuaState) -> i32 {
	match get_iface() {
		Ok(iface) => {
			let paused = iface.IsPaused();
			lua_pushboolean(l, paused as i32);
			return 1;
		},
		Err(e) => {
			printgm!(l, "Failed to get interface: {}", e);
		}
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
