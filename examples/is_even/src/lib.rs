use rglua::prelude::*;

// The functions we want to provide to lua
extern "C" fn is_even(l: LuaState) -> i32 {
	let num = luaL_checkinteger(l, 1);
	// Ask for the first argument of the function.
	// If this is the wrong type or missing, an error will be thrown to lua (if you don't want this, use the lua_to* functions)

	lua_pushboolean(l, (num % 2 == 0) as i32);

	// This returns one value
	1
}

extern "C" fn is_odd(l: LuaState) -> i32 {
	let num = luaL_checkinteger(l, 1);

	lua_pushboolean(l, (num % 2 != 0) as i32);
	1
}

#[gmod_open]
fn open(l: LuaState) -> i32 {
	// Print to the gmod console
	printgm!(l, "Loaded is_even module!");

	// Create a library to organize all of our functions to export to gmod.
	let lib = reg! [
		"is_even" => is_even,
		"is_odd" => is_odd
	];

	// Register our functions in ``_G.math``
	// This WILL NOT overwrite _G.math if it already exists ()
	luaL_register(l, cstr!("math"), lib.as_ptr());
	1
}

#[gmod_close]
fn close(l: LuaState) -> i32 {
	printgm!(l, "Goodbye garrysmod!");
	0
}