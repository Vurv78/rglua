use rglua::prelude::*;

#[derive(Debug, thiserror::Error)]
enum ResultError {
	#[error("Number was negative!")]
	Negative,
	#[error("Number greater than 5!")]
	Catastrophic
}

#[lua_function]
fn result(l: LuaState) -> Result<i32, ResultError> {
	let num = luaL_checkinteger(l, 1);

	if num > 5 {
		return Err( ResultError::Catastrophic )
	} else if num < 0 {
		return Err( ResultError::Negative )
	} else {
		lua_pushinteger(l, num);
		return Ok(1)
	}
}

#[gmod_open]
fn open(l: LuaState) -> i32 {
	printgm!(l, "Loaded exception module!");

	let lib = reg! [
		// "panic" => panic, (Soon™️)
		"result" => result
	];

	luaL_register(l, cstr!("except"), lib.as_ptr());
	0
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
	0
}
