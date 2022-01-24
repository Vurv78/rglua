use rglua::prelude::*;
use rglua_macros::{gmod_close, gmod_open, lua_function};

#[derive(Debug)]
enum LuaError {}


// The errors you return must implement display, to be relayed to gmod.
use std::fmt::{Display, Formatter};
impl Display for LuaError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "LuaError")
	}
}

#[lua_function]
fn add(_state: LuaState) -> Result<i32, LuaError> {
	Ok(0)
}

#[gmod_open]
fn entry(_state: LuaState) -> Result<i32, LuaError> {
	println!("Hello world!");
	Ok(0)
}

#[gmod_close]
fn close(_s: LuaState) -> Result<i32, LuaError> {
	Ok(0)
}

// Appease test runner
fn main() {}
