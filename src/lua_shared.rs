#![allow(unused)]

use libloading::Library;

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

macro_rules! dyn_symbols {
	( $vis:vis extern fn $name:ident( $($arg:ident : $argty:ty),* ) -> $ret:ty; $($rest:tt)* ) => {
		$vis static $name: Lazy<extern fn($($arg: $argty),*) -> $ret> = Lazy::new(|| {
			unsafe {
				let lib = &*LUA_SHARED_RAW;
				let v: libloading::Symbol<extern fn($($arg: $argty),*) -> $ret> = lib.get( stringify!($name).as_bytes() ).unwrap();
				std::mem::transmute(v)
			}
		});
		dyn_symbols!( $($rest)* );
	};

	() => ()
}

pub static LUA_SHARED_RAW: Lazy<Library> = Lazy::new(|| {
	let path = LUA_SHARED_PATH.as_ref().expect("Couldn't find lua_shared.dll!");
	unsafe { Library::new(path).expect("Could not open library") }
});

use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::types::*;
use crate::globals::Lua::{self, GLOBALSINDEX};

pub type LuaJITProfileCallback = extern "C" fn(data: *mut c_void, l: LuaState, samples: c_int, vmstate: c_int) -> ();

// Create Lazy cells that'll find the functions at runtime when called.
dyn_symbols!(
	pub extern fn CreateInterface(pName: LuaString, pReturnCode: *mut c_int) -> *mut c_void;

	pub extern fn luaL_loadbufferx(state: LuaState, code: LuaString, size: SizeT, id: LuaString, mode: LuaString) -> c_int;
	pub extern fn luaL_loadbuffer(state: LuaState, code: LuaString, size: SizeT, id: LuaString) -> c_int;
	pub extern fn luaL_loadstring(state: LuaState, code: LuaString) -> c_int;
	pub extern fn luaL_loadfile(state: LuaState, filename: LuaString) -> c_int;
	pub extern fn luaL_loadfilex(state: LuaState, filename: LuaString, mode: LuaString) -> c_int;

	// Call lua code
	pub extern fn lua_pcall(state: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int;
	pub extern fn lua_call(state: LuaState, nargs: c_int, nresults: c_int) -> c_int;
	pub extern fn lua_cpcall(state: LuaState, func: LuaCFunction, userdata: *mut c_void ) -> c_int;
	pub extern fn luaL_callmeta(state: LuaState, obj: c_int, name: LuaString) -> c_int;

	// Setters
	pub extern fn lua_setfield(state: LuaState, idx: c_int, name: LuaString) -> ();

	pub extern fn lua_setmetatable(state: LuaState, idx: c_int) -> ();
	pub extern fn lua_settop(state: LuaState, ind: c_int) -> ();
	pub extern fn lua_setupvalue(state: LuaState, fidx: c_int, idx: c_int) -> LuaString;
	pub extern fn lua_setfenv(state: LuaState, idx: c_int) -> c_int;
	pub extern fn lua_settable(state: LuaState, idx: c_int) -> ();
	pub extern fn lua_rawset(state: LuaState, idx: c_int) -> (); // lua_settable but no metamethods called
	pub extern fn lua_rawseti(state: LuaState, idx: c_int, n: c_int) -> (); // t[n] = v

	// Getters
	pub extern fn lua_gettable(state: LuaState, idx: c_int) -> ();
	pub extern fn lua_rawget(state: LuaState, idx: c_int) -> (); // lua_gettable but no metamethods called
	pub extern fn lua_rawgeti(state: LuaState, idx: c_int, n: c_int) -> (); // lua_gettable but no metamethods called

	pub extern fn lua_getfield(state: LuaState, idx: c_int, key: LuaString) -> ();
	pub extern fn lua_getupvalue(state: LuaState, fidx: c_int, idx: c_int) -> LuaString;
	pub extern fn lua_type(state: LuaState, idx: c_int) -> c_int;
	pub extern fn lua_typename(state: LuaState, typeid: c_int) -> LuaString; // To be used with the return value of lua_type

	// Getters (with "to")
	pub extern fn lua_tolstring(state: LuaState, ind: c_int, size: SizeT) -> LuaString;
	pub extern fn lua_toboolean(state: LuaState, idx: c_int) -> c_int;
	pub extern fn lua_tocfunction(state: LuaState, idx: c_int) -> LuaCFunction;
	pub extern fn lua_tointeger(state: LuaState, idx: c_int) -> LuaInteger;
	pub extern fn lua_tonumber(state: LuaState, idx: c_int) -> LuaNumber;
	pub extern fn lua_topointer(state: LuaState, idx: c_int) -> *mut c_void;
	pub extern fn lua_tothread(state: LuaState, idx: c_int) -> LuaState;
	pub extern fn lua_touserdata(state: LuaState, idx: c_int) -> *mut c_void;

	// Push functions
	pub extern fn lua_pushstring(state: LuaState, s: LuaString) -> ();
	pub extern fn lua_pushboolean(state: LuaState, s: c_int) -> ();
	pub extern fn lua_pushlstring(state: LuaState, s: LuaString, sz: SizeT) -> ();
	pub extern fn lua_pushnil(state: LuaState) -> ();
	pub extern fn lua_pushnumber(state: LuaState, num: LuaNumber) -> ();
	pub extern fn lua_pushvalue(state: LuaState, idx: c_int) -> ();
	pub extern fn lua_pushcclosure(state: LuaState, fnc: LuaCFunction, idx: c_int) -> ();
	pub extern fn lua_pushlightuserdata(state: LuaState, p: *mut c_void) -> ();
	pub extern fn lua_pushthread(state: LuaState) -> ();
	//pub extern fn lua_pushfstring(state: LuaState, fmt: LuaString, ...) -> LuaString;
	pub extern fn lua_pushinteger(state: LuaState, n: LuaInteger) -> ();

	// Type Checks
	pub extern fn luaL_checkinteger(state: LuaState, narg: c_int) -> LuaInteger;
	pub extern fn luaL_checknumber(state: LuaState, narg: c_int) -> LuaNumber;
	pub extern fn luaL_checklstring(state: LuaState, narg: c_int, len: SizeT) -> LuaString;

	// Type Checks that return nothing
	pub extern fn luaL_checkstack(state: LuaState, size: c_int, msg: LuaString) -> ();
	pub extern fn luaL_checkany(state: LuaState, narg: c_int) -> ();
	pub extern fn luaL_checktype(state: LuaState, narg: c_int, typeid: c_int) -> ();
	pub extern fn luaL_checkudata(state: LuaState, narg: c_int, len: SizeT) -> ();

	// Creation
	pub extern fn luaL_newstate() -> LuaState;
	pub extern fn lua_createtable(state: LuaState, narr: c_int, nrec: c_int) -> ();

	// Destruction
	pub extern fn lua_close(state: LuaState) -> (); // Destroys the lua state

	// JIT
	// Returns 1 for success, 0 for failure
	pub extern fn luaJIT_setmode(state: LuaState, idx: c_int, jit_mode: c_int) -> c_int;
	pub extern fn luaJIT_profile_stop(state: LuaState) -> ();

	pub extern fn luaJIT_profile_start(state: LuaState, mode: LuaString, cb: LuaJITProfileCallback, data: *mut c_void) -> ();
	pub extern fn luaJIT_profile_dumpstack(state: LuaState, fmt: LuaString, depth: c_int, len: SizeT) -> LuaString;

	// Coroutines
	pub extern fn lua_yield(state: LuaState, nresults: c_int) -> c_int;
	pub extern fn lua_status(state: LuaState) -> c_int;
	pub extern fn lua_resume_real(state: LuaState, narg: c_int) -> c_int;

	// Comparison
	pub extern fn lua_equal(state: LuaState, ind1: c_int, ind2: c_int) -> c_int; // Returns 1 or 0 bool
	pub extern fn lua_rawequal(state: LuaState, ind1: c_int, ind2: c_int) -> c_int;

	// Raising Errors
	pub extern fn luaL_typerror(state: LuaState, narg: c_int, typename: LuaString) -> c_int;
	//pub extern fn luaL_error(state: LuaState, fmt: LuaString, ...) -> c_int;
	pub extern fn luaL_argerror(state: LuaState, narg: c_int, extramsg: LuaString) -> c_int;
	pub extern fn lua_error(state: LuaState) -> c_int;

	// Open
	pub extern fn luaopen_table(state: LuaState) -> c_int;
	pub extern fn luaopen_string(state: LuaState) -> c_int;
	pub extern fn luaopen_package(state: LuaState) -> c_int;
	pub extern fn luaopen_os(state: LuaState) -> c_int;
	pub extern fn luaopen_math(state: LuaState) -> c_int;
	pub extern fn luaopen_jit(state: LuaState) -> c_int;
	pub extern fn luaopen_debug(state: LuaState) -> c_int;
	pub extern fn luaopen_bit(state: LuaState) -> c_int;
	pub extern fn luaopen_base(state: LuaState) -> c_int;
	pub extern fn luaL_openlib(state: LuaState) -> c_int;

	// Ref
	pub extern fn luaL_ref(state: LuaState, t: c_int) -> c_int;
	pub extern fn luaL_unref(state: LuaState, t: c_int, r: c_int) -> ();

	// Metatables
	pub extern fn luaL_newmetatable(state: LuaState, tname: LuaString) -> c_int;
	pub extern fn luaL_newmetatable_type(state: LuaState, tname: LuaString, typ: c_int) -> c_int;
	pub extern fn luaL_getmetafield(state: LuaState, obj: c_int, e: LuaString) -> c_int;

	// Optional / Default to ``d``
	pub extern fn luaL_optinteger(state: LuaState, narg: c_int, d: LuaInteger) -> c_int;
	pub extern fn luaL_optlstring(state: LuaState, arg: c_int, d: LuaString, l: SizeT) -> LuaString;
	pub extern fn luaL_optnumber(state: LuaState, arg: c_int, d: LuaNumber) -> LuaNumber;

	// x / ref functions
	pub extern fn lua_tointegerx(state: LuaState, index: c_int, isnum: *mut c_int) -> LuaInteger;
	pub extern fn lua_tonumberx(state: LuaState, index: c_int, isnum: *mut c_int) -> LuaNumber;

	// Debug
	pub extern fn luaL_traceback(state: LuaState, state1: LuaState, msg: LuaString, level: c_int) -> ();
	pub extern fn luaL_where(state: LuaState, lvl: c_int) -> ();

	// Misc
	pub extern fn luaL_testudata(state: LuaState, arg: c_int, tname: LuaString) -> ();
	pub extern fn luaL_execresult(state: LuaState, stat: c_int) -> c_int;
	pub extern fn luaL_fileresult(state: LuaState, stat: c_int, fname: LuaString) -> c_int;
	pub extern fn luaL_findtable(state: LuaState, idx: c_int, fname: LuaString, szhint: c_int) -> LuaString;
	pub extern fn lua_checkstack(state: LuaState, extra: c_int) -> c_int;
	pub extern fn lua_atpanic(state: LuaState, panicf: LuaCFunction) -> LuaCFunction;
	pub extern fn lua_gettop(state: LuaState) -> c_int;

	// luaL_Buffer
	pub extern fn luaL_buffinit(state: LuaState, b: *mut LuaL_Buffer) -> ();
	pub extern fn luaL_prepbuffer(b: *mut LuaL_Buffer) -> *mut i8;

	// String methods
	pub extern fn luaL_gsub(s: LuaString, pattern: LuaString, replace: LuaString) -> LuaString;
);

#[inline(always)]
pub fn lua_pop(state: LuaState, ind: c_int) {
	lua_settop( state, -(ind)-1 );
}

#[inline(always)]
pub fn lua_getglobal(state: LuaState, name: LuaString) {
	lua_getfield(state, GLOBALSINDEX, name);
}

#[inline(always)]
pub fn lua_setglobal(state: LuaState, name: LuaString) {
	lua_setfield(state, GLOBALSINDEX, name);
}

#[inline(always)]
pub fn lua_pushcfunction(state: LuaState, fnc: LuaCFunction) {
	lua_pushcclosure(state, fnc, 0);
}

#[inline(always)]
pub fn lua_tostring(state: LuaState, idx: c_int) -> LuaString {
	lua_tolstring(state, idx, 0)
}

#[inline(always)]
pub fn lua_resume(state: LuaState, narg: c_int) -> c_int {
	lua_resume_real(state, narg)
}