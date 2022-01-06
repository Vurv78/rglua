pub(crate) use super::prelude::{self, interfaces, VTable};

interfaces! {
	#[version("")]
	#[file("")]
	/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaObject.h#L24>
	/// You do not use this as a typical interface, it is just a type returned by other iface functions.
	pub abstract struct ILuaObject {};

	#[version("")]
	#[file("")]
	/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaInterface.h#L25>
	/// Basically what is given to ordinary C++ binary modules that do not interface with lua_shared.
	/// You can use this but should really just use the lua_shared bindings.
	pub abstract struct ILuaInterface {};

	#[version("LUASHARED003")]
	#[file("")]
	/// <https://github.com/danielga/garrysmod_common/blob/9981d4aaee15452a9b0f53436c1aa807f81f3fd6/include/GarrysMod/Lua/LuaShared.h#L57>
	pub abstract struct CLuaShared {};
}

mod interface;
mod shared;
