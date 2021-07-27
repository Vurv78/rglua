
use std::ffi::{ c_void, CString };

pub use crate::interfaces::*;

pub type CreateInterfaceFn = extern "system" fn(
	pName: *const i8,
	pReturnCode: *mut i32
) -> *mut c_void;

pub unsafe fn get_interface_handle(file: &str) -> Result<CreateInterfaceFn, libloading::Error> {
	let lib = libloading::Library::new(file)?;
	let sym: libloading::Symbol<CreateInterfaceFn> = lib.get(b"CreateInterface\0")?;

	Ok( std::mem::transmute(sym) )
}

#[derive(Debug)]
pub enum InterfaceError {
	BadCString( std::ffi::NulError ),
	FactoryNotFound,
}

pub fn get_from_interface(iface: &str, factory: CreateInterfaceFn) -> Result<*mut (), InterfaceError> {
	let mut status = 0;

	let iface = CString::new(iface)
		.map_err( |e| InterfaceError::BadCString(e) )?;

	let result = factory( iface.as_ptr(), &mut status );

	if status == 0 && !result.is_null() {
		Ok(result as *mut ())
	} else {
		Err( InterfaceError::FactoryNotFound )
	}
}