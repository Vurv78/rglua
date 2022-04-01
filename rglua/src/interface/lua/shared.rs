use super::interface::LuaInterface;
use super::prelude::*;
/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaShared.h#L57>
/// This doesn't work on x86. =(
#[vtable]
pub struct LuaShared {
	#[skip(2)] // ~LuaShared, Init
	pub Shutdown: extern "C" fn(),
	pub DumpStats: extern "C" fn(),
	pub CreateLuaInterface: extern "C" fn(realm: c_uchar, b: bool) -> *mut LuaInterface,
	pub CloseLuaInterface: extern "C" fn(iface: *mut LuaInterface),

	/// 0 - Client
	/// 1 - Server
	/// 2 - Menu
	pub GetLuaInterface: extern "C" fn(realm: c_uchar) -> *mut LuaInterface,
	#[skip(2)] // LoadFile, GetCache
	pub MountLua: extern "C" fn(l: *const c_char),
	pub MountLuaAdd: extern "C" fn(l: *const c_char, l2: *const c_char),
	pub UnMountLua: extern "C" fn(l: *const c_char),
	pub SetFileContents: extern "C" fn(l: *const c_char, c: *const c_char),
	pub SetLuaFindHook: extern "C" fn(p: *mut c_void),
	#[skip(1)] // FindScripts
	pub GetStackTraces: extern "C" fn() -> *const c_char,
	#[skip(1)] // InvalidateCache
	pub EmptyCache: extern "C" fn(),
}
