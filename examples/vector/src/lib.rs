use rglua::prelude::*;

#[lua_function]
fn is_positive(l: LuaState) -> i32 {
	let vec = luaL_checkvector(l, 1);
	lua_pushboolean(l, (vec.x > 0.0 && vec.y > 0.0 && vec.z > 0.0) as i32);
	1
}

#[lua_function]
fn get_pow(l: LuaState) -> i32 {
	let vec = luaL_checkvector(l, 1);
	let by = luaL_checknumber(l, 2) as f32;

	lua_pushvector(l, Vector::new(vec.x.powf(by), vec.y.powf(by), vec.z.powf(by)));
	1
}

// Note that since this is #[gmod_open] the name of the function does not matter
// This is the same for #[gmod_close]
#[gmod_open]
fn open(l: LuaState) -> i32 {
	// Create a library consisting of functions to export to gmod.
	let lib = reg! [
		"IsPositive" => is_positive,
		"GetPow" => get_pow
	];

	// Get the ``Vector`` metatable from the lua registry and put it onto the stack.
	luaL_getmetatable(l, cstr!("Vector"));

	// Give a null pointer as the libname so that luaL_register knows we are trying to instead add these functions to the value on top of the stack;
	// This being the Vector metatable at (-1).
	luaL_register(l, std::ptr::null(), lib.as_ptr());

	// Return nothing (0 objects)
	0
}

#[gmod_close]
fn close(l: LuaState) -> i32 {
	printgm!(l, "Goodbye garrysmod!");
	0
}