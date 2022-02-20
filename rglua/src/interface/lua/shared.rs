use super::interface::LuaInterface;
use super::prelude::*;
/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaShared.h#L57>
/// This doesn't work on x86. =(
#[vtable]
pub struct LuaShared {
	#[offset(2)] // 2
	pub Shutdown: extern "C" fn(),
	pub DumpStats: extern "C" fn(),
	pub CreateLuaInterface: extern "C" fn(realm: c_uchar, b: bool) -> *mut LuaInterface,
	pub CloseLuaInterface: extern "C" fn(iface: *mut LuaInterface),

	/// 0 - Client
	/// 1 - Server
	/// 2 - Menu
	pub GetLuaInterface: extern "C" fn(realm: c_uchar) -> *mut LuaInterface,

	#[offset(9)]
	pub MountLua: extern "C" fn(l: *const c_char),
	pub MountLuaAdd: extern "C" fn(l: *const c_char, l2: *const c_char),
	pub UnMountLua: extern "C" fn(l: *const c_char),
	pub SetFileContents: extern "C" fn(l: *const c_char, c: *const c_char)
}
