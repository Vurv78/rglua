use super::prelude::*;
use super::{ILuaInterface, ILuaObject};

impl ILuaInterface {
	#[virtual_index(3)]
	pub fn Global(&self) -> *mut ILuaObject {}

	#[virtual_index(4)]
	pub fn GetObject(&self, index: c_int) -> *mut ILuaObject {}

	#[virtual_index(20)]
	pub fn IsServer(&self) -> bool {}

	#[virtual_index(21)]
	pub fn IsClient(&self) -> bool {}

	#[virtual_index(22)]
	pub fn IsMenu(&self) -> bool {}
}
