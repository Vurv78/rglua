use super::base::LuaBase;
use super::prelude::*;
use super::LuaObject;

/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaInterface.h#L25>
/// Basically what is given to ordinary C++ binary modules that do not interface with lua_shared.
/// You can use this but should really just use the lua_shared bindings.
#[vtable]
pub struct LuaInterface {
	pub base: *mut *mut LuaBase,

	#[offset(1)]
	pub Shutdown: extern "C" fn(),
	pub Cycle: extern "C" fn(),

	#[offset(3)]
	pub Global: extern "C" fn() -> *mut LuaObject,
	pub GetObject: extern "C" fn(index: c_int) -> *mut LuaObject,
	pub PushLuaObject: extern "C" fn(o: *mut LuaObject),
	pub PushLuaFunction: extern "C" fn(f: crate::types::LuaCFunction),
	pub LuaError: extern "C" fn(err: *const c_char, idx: c_int),
	pub TypeError: extern "C" fn(name: *const c_char, idx: c_int),
	pub CallInternal: extern "C" fn(args: c_int, rets: c_int),

	#[offset(20)]
	pub IsServer: extern "C" fn() -> bool,

	#[offset(21)]
	pub IsClient: extern "C" fn() -> bool,

	#[offset(22)]
	pub IsMenu: extern "C" fn() -> bool,

	#[offset(37)]
	pub RunString: extern "C" fn(
		filename: *const c_char,
		path: *const c_char,
		code: *const c_char,
		run: bool,
		show_errors: bool
	) -> bool,

	#[offset(39)]
	pub Error: extern "C" fn(err: *const c_char),

	#[offset(45)]
	pub ErrorNoHalt: extern "C" fn(fmt: *const c_char, ...),
	pub Msg: extern "C" fn(fmt: *const c_char, ...)
}
