pub(crate) mod prelude {
	pub(crate) use vtables::VTable;
	pub(crate) use vtables_derive::*;

	pub(crate) use crate::interface::common::PlayerInfo;
}

mod common;
mod engine;
mod lua;
mod panel;

pub use engine::EngineClient;
pub use lua::{ILuaInterface, CLuaShared};
pub use panel::{IPanel};

use crate::try_cstr;
use std::ffi::{c_void, CString};
use libloading::{Library, Symbol};

pub type CreateInterfaceFn = extern "system" fn(pName: *const i8, pReturnCode: *mut i32) -> *mut c_void;

///  # Safety
/// This function is unsafe to transmute the internal libloading symbol to a proper createinterface function pointer.
pub unsafe fn get_interface_handle(file: &str) -> Result<CreateInterfaceFn, libloading::Error> {
	let lib = Library::new(file)?;
	let sym: Symbol<CreateInterfaceFn> = lib.get(b"CreateInterface\0")?;

	Ok(std::mem::transmute(sym))
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterfaceError {
	#[error("Failed to convert interface to C String")]
	BadCString(#[from] std::ffi::NulError),
	#[error("Couldn't find factory at this interface!")]
	FactoryNotFound,
}

pub fn get_from_interface(
	iface: &str,
	factory: CreateInterfaceFn,
) -> Result<*mut (), InterfaceError> {
	let mut status = 0;

	let iface = try_cstr!(iface)?;

	let result = factory(iface, &mut status);

	if status == 0 && !result.is_null() {
		Ok(result as *mut ())
	} else {
		Err(InterfaceError::FactoryNotFound)
	}
}
