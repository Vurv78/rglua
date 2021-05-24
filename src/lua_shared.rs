#![allow(unused)]
#![macro_export]

use dlopen::raw::Library;

// Keep separate in case needed by crates.
pub static GMOD_DIR: Lazy<PathBuf> = Lazy::new(|| {
	// Get the attached process. If you inject or run a binary module, will always GarrysMod directory. If not then you did something wrong.
	std::env::current_dir().expect("Couldn't get current running directory.") // D:\SteamLibrary\steamapps\common\GarrysMod for example.
});

// Let me know if there's a neater way to do this.
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let gmod_dir = &*GMOD_DIR;
	let mut full: PathBuf;

	if cfg!( target_arch = "x86_64" ) {
		// x64 Platform. Always should be in GMOD/bin/win64
		full = gmod_dir
			.join("bin")
			.join("win64")
			.join("lua_shared.dll");
	} else {
		// x86 Platform, Either in GMOD/garrysmod/bin or GMOD/bin
		full = gmod_dir
			.join("garrysmod")
			.join("bin")
			.join("lua_shared.dll");
		if !full.exists() {
			full = gmod_dir
				.join("bin")
				.join("lua_shared.dll");
		}
	}
	if !full.exists() {
		eprintln!( "lua_shared.dll couldn't be found! Tried to get {}", full.display() );
		return None;
	}

	Some(full)
});

macro_rules! expose_symbol {
	($name:ident, $ret:ty, $($args:tt)*) => {
		pub const $name: Lazy<extern fn$($args)* -> $ret> = Lazy::new(|| {
			unsafe { LUA_SHARED_RAW.symbol( stringify!($name) ) }.unwrap()
		});
	};
}

// dlopen raw lua_shared.dll library. Don't use this unless you know what you're doing.
pub const LUA_SHARED_RAW: Lazy<Library> = Lazy::new(|| {
	let path = LUA_SHARED_PATH.as_ref().expect("Couldn't find lua_shared.dll!");
	Library::open(path).expect("Could not open library")
});

use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::types::*;
use crate::globals::Lua::GLOBALSINDEX;

// Load lua Code
expose_symbol!( luaL_loadbufferx, CInt, (state: LuaState, code: CharBuf, size: SizeT, id: CharBuf, mode: CharBuf) );
expose_symbol!( luaL_loadbuffer, CInt, (state: LuaState, code: CharBuf, size: SizeT, id: CharBuf) );
expose_symbol!( luaL_loadstring, CInt, (state: LuaState, code: CharBuf) );

// Call lua code
expose_symbol!( lua_pcall, CInt, (state: LuaState, nargs: CInt, nresults: CInt, msgh: CInt) );
expose_symbol!( lua_call, CInt, (state: LuaState, nargs: CInt, nresults: CInt) );
expose_symbol!( lua_cpcall, CInt, (state: LuaState, func: LuaCFunction, userdata: *mut CVoid ) );
expose_symbol!( luaL_callmeta, CInt, (state: LuaState, obj: CInt, name: CharBuf) );

// Setters
expose_symbol!( lua_setfield, (), (state: LuaState, idx: CInt, name: CharBuf) );

expose_symbol!( lua_setmetatable, (), (state: LuaState, idx: CInt) );
expose_symbol!( lua_settop, (), (state: LuaState, ind: CInt) );
expose_symbol!( lua_setupvalue, CharBuf, (state: LuaState, fidx: CInt, idx: CInt) );
expose_symbol!( lua_setfenv, CInt, (state: LuaState, idx: CInt) );
expose_symbol!( lua_settable, (), (state: LuaState, idx: CInt) );
expose_symbol!( lua_rawset, (), (state: LuaState, idx: CInt) ); // lua_settable but no metamethods called
expose_symbol!( lua_rawseti, (), (state: LuaState, idx: CInt, n: CInt) ); // t[n] = v

// Getters
expose_symbol!( lua_gettable, (), (state: LuaState, idx: CInt) );
expose_symbol!( lua_rawget, (), (state: LuaState, idx: CInt) ); // lua_gettable but no metamethods called
expose_symbol!( lua_rawgeti, (), (state: LuaState, idx: CInt, n: CInt) ); // lua_gettable but no metamethods called

expose_symbol!( lua_getfield, (), (state: LuaState, idx: CInt, key: CharBuf) );
expose_symbol!( lua_getupvalue, CharBuf, (state: LuaState, fidx: CInt, idx: CInt) );
expose_symbol!( lua_type, CInt, (state: LuaState, idx: CInt) );
expose_symbol!( lua_typename, CharBuf, (state: LuaState, typeid: CInt) ); // To be used with the return value of lua_type

// Getters (with "to")
expose_symbol!( lua_tolstring, CharBuf, (state: LuaState, ind: CInt, size: SizeT) );
expose_symbol!( lua_toboolean, CInt, (state: LuaState, idx: CInt) );
expose_symbol!( lua_tocfunction, LuaCFunction, (state: LuaState, idx: CInt) );
expose_symbol!( lua_tointeger, LuaInteger, (state: LuaState, idx: CInt) );
expose_symbol!( lua_tonumber, LuaNumber, (state: LuaState, idx: CInt) );
expose_symbol!( lua_topointer, *mut CVoid, (state: LuaState, idx: CInt) );
expose_symbol!( lua_tothread, LuaState, (state: LuaState, idx: CInt) );
expose_symbol!( lua_touserdata, *mut CVoid, (state: LuaState, idx: CInt) );

// Push functions
expose_symbol!( lua_pushstring, (), (state: LuaState, s: CharBuf) );
expose_symbol!( lua_pushboolean, (), (state: LuaState, s: CInt) );
expose_symbol!( lua_pushlstring, (), (state: LuaState, s: CharBuf, sz: SizeT) );
expose_symbol!( lua_pushnil, (), (state: LuaState) );
expose_symbol!( lua_pushnumber, (), (state: LuaState, num: LuaNumber) );
expose_symbol!( lua_pushvalue, (), (state: LuaState, idx: CInt) );
expose_symbol!( lua_pushcclosure, (), (state: LuaState, fnc: LuaCFunction, idx: CInt) );
expose_symbol!( lua_pushlightuserdata, (), (state: LuaState, p: *mut CVoid) );
expose_symbol!( lua_pushthread, (), (state: LuaState) );
expose_symbol!( lua_pushfstring, CharBuf, (state: LuaState, fmt: CharBuf, ...) );

// Type Checks
expose_symbol!( luaL_checkinteger, LuaInteger, (state: LuaState, narg: CInt) );
expose_symbol!( luaL_checknumber, LuaNumber, (state: LuaState, narg: CInt) );
expose_symbol!( luaL_checklstring, CharBuf, (state: LuaState, narg: CInt) );

// Type Checks that return nothing
expose_symbol!( luaL_checkstack, (), (state: LuaState, size: CInt, msg: CharBuf) );
expose_symbol!( luaL_checkany, (), (state: LuaState, narg: CInt) );
expose_symbol!( luaL_checktype, (), (state: LuaState, narg: CInt, typeid: CInt) );
expose_symbol!( luaL_checkudata, (), (state: LuaState, narg: CInt, len: SizeT) );

// Creation
expose_symbol!( luaL_newstate, LuaState, () );
expose_symbol!( lua_createtable, (), (state: LuaState, narr: CInt, nrec: CInt) );

// Destruction
expose_symbol!( lua_close, (), (state: LuaState) ); // Destroys the lua state

// JIT
// Returns 1 for success, 0 for failure
expose_symbol!( luaJIT_setmode, CInt, (state: LuaState, idx: CInt, jit_mode: CInt) );

// Coroutines
expose_symbol!( lua_yield, CInt, (state: LuaState, nresults: CInt) );
expose_symbol!( lua_status, CInt, (state: LuaState) );
expose_symbol!( lua_resume_real, CInt, (state: LuaState, narg: CInt) );

// Comparison
expose_symbol!( lua_equal, CInt, (state: LuaState, ind1: CInt, ind2: CInt) ); // Returns 1 or 0 bool
expose_symbol!( lua_rawequal, CInt, (state: LuaState, ind1: CInt, ind2: CInt) );

// Raising Errors
expose_symbol!( luaL_typerror, CInt, (state: LuaState, narg: CInt, typename: CharBuf) );

#[macro_export]
macro_rules! lua_pop {
	($state:expr, $ind:literal) => {
		rglua::lua_shared::lua_settop( $state, -($ind)-1 );
	}
}

#[macro_export]
macro_rules! lua_getglobal {
	($state:expr, $name:expr) => {
		rglua::lua_shared::lua_getfield($state, rglua::globals::Lua::GLOBALSINDEX, $name);
	}
}

#[macro_export]
macro_rules! lua_setglobal {
	($state:expr, $name:expr) => {
		rglua::lua_shared::lua_setfield($state, rglua::globals::Lua::GLOBALSINDEX, $name);
	}
}

#[macro_export]
macro_rules! lua_pushcfunction {
	($state:expr, $fnc:expr) => {
		rglua::lua_shared::lua_pushcclosure($state, $fnc, 0);
	}
}

#[macro_export]
macro_rules! lua_tostring {
	($state:expr, $idx:expr) => {
		rglua::lua_shared::lua_tolstring($state, $idx, 0)
	}
}

#[macro_export]
macro_rules! lua_resume {
	($state:expr, $narg:expr) => {
		rglua::lua_shared::lua_resume_real($state, $narg)
	}
}