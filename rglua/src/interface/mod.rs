#![allow(non_snake_case)]

pub(crate) mod prelude {
	pub(crate) use viable::vtable;

	pub(crate) use crate::interface::common::PlayerInfo;
	pub(crate) use std::os::raw::{
		c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ushort, c_void
	};

	#[cfg(feature = "userdata")]
	pub(crate) use crate::userdata::Vector;
}

mod common;
mod cvar;
mod engine;
mod lua;
mod materials;
mod mdl;
mod net;
mod panel;

pub use cvar::{CVar, ConVar};
pub use engine::{EngineClient, EngineServer};
pub use lua::{LuaInterface, LuaObject, LuaShared};
pub use materials::MaterialSystem;
pub use mdl::{MdlCache, MdlCacheNotify};
pub use net::NetChannelInfo;
pub use panel::Panel;

use crate::try_cstr;
use libloading::{Library, Symbol};
use std::ffi::c_void;

pub type CreateInterfaceFn =
	extern "system" fn(pName: *const i8, pReturnCode: *mut i32) -> *mut c_void;

/// Gets a handle to provided source interface
/// You should really use the [iface] macro instead
/// # Arguments
/// * `file` - Filename of the interface dll, linked to gmod. For example "engine.dll"
/// # Safety
/// This function internally gets the symbol to the CreateInterface function and casts it to the desired interface provided
/// So make sure you pass the correct interface type and a valid dll.
/// # Examples
/// ```rust, no_run
/// use rglua::interface::get_interface_handle;
/// unsafe {
///     let vgui = get_interface_handle("vgui2.dll")
///         .expect("Couldn't link to vgui2.dll");
/// };
/// ```
pub unsafe fn get_interface_handle(file: &str) -> Result<CreateInterfaceFn, libloading::Error> {
	let lib = Library::new(file)?;
	let sym: Symbol<CreateInterfaceFn> = lib.get(b"CreateInterface\0".as_ref())?;

	Ok(*sym)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Libloading error: {0}")]
	Libloading(#[from] libloading::Error),

	#[error("Failed to convert interface to c string. {0}")]
	BadCString(#[from] std::ffi::NulError),

	#[error("Couldn't find factory of interface {0}!")]
	FactoryNotFound(String),

	#[error("Failure in CreateInterface of interface {1}; Code [{0}]")]
	CreateInterface(i32, String),

	#[error("Failed to get interface {0} as mutable")]
	IFaceMut(String),

	#[error("Failed to get object as mutable")]
	AsMut
}

/// Tries to get source interface from given interface name, and handle to it acquired from [get_interface_handle]
/// You should really use the [iface] macro instead
/// # Arguments
/// * `iface` - name of the interface to get, for example "VGUI_Panel009"
/// * `factory` - handle to the interface, acquired from [get_interface_handle]
/// # Examples
/// Getting the raw PaintTraverse function from vgui:
/// ```no_run
/// // Wrappers to these interfaces are already provided but they do not give raw function pointers which is needed to detour / modify the functions
/// // in any way, which you may want to do here, especially for painttraverse since you can safely run lua here if you queue it from a thread to avoid crashes.
/// use rglua::{prelude::*, interface::Panel};
/// type PaintTraverseFn = extern "fastcall" fn(&'static Panel, usize, bool, bool);
/// let vgui = iface!(Panel).expect("Couldn't get VGUI interface");
/// // Transmute the function address from the offset into our known signature
/// // You should use Interface.get_raw for this though
/// let paint_traverse: PaintTraverseFn = unsafe {
///     std::mem::transmute(
///         (vgui.vtable as *mut *mut std::ffi::c_void)
///             .offset(41) // vtable offset as seen in [interface/panel.rs]
///             .read()
///     )
/// };
/// ```
pub fn get_from_interface(iface: &str, factory: CreateInterfaceFn) -> Result<*mut (), Error> {
	let mut status = 0;

	let interface = try_cstr!(iface)?;
	let result = factory(interface.as_ptr(), &mut status);

	if status == 0 && !result.is_null() {
		Ok(result as *mut ())
	} else {
		Err(Error::FactoryNotFound(iface.to_owned()))
	}
}
