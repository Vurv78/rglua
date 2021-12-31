#![allow(non_snake_case)]

pub(crate) mod prelude {
	pub(crate) use vtables::VTable;
	pub(crate) use vtables_derive::*;

	pub(crate) use crate::interface::common::PlayerInfo;
	pub(crate) use std::os::raw::{c_char, c_int, c_uchar, c_ushort, c_void};

	#[cfg(feature = "userdata")]
	pub(crate) use crate::userdata::Vector;

	macro_rules! iface {
		(
			#[ version ( $ver:literal ) ]
			#[ file ( $file:literal ) ]
			$(#[$attr:meta])*
			$vis:vis abstract struct $iface:ident {};
			$($rest:tt)*
		) => {
			$(#[$attr])*
			#[derive(VTable)]
			#[vtables_derive::has_vtable]
			$vis struct $iface {
				pub vtable: usize
			}
			iface!( $($rest)* );
		};
		() => ();
	}

	pub(crate) use iface;
}

mod common;
mod cvar;
mod engine;
mod lua;
mod materials;
mod mdl;
mod panel;

pub use cvar::ICVar;
pub use engine::EngineClient;
pub use lua::{CLuaShared, ILuaInterface, ILuaObject};
pub use materials::IMaterialSystem;
pub use mdl::{IMdlCache, IMdlCacheNotify};
pub use panel::IPanel;

use crate::try_cstr;
use libloading::{Library, Symbol};
use std::ffi::c_void;

pub type CreateInterfaceFn =
	extern "system" fn(pName: *const i8, pReturnCode: *mut i32) -> *mut c_void;

/// Gets a handle to provided source interface
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

/// Tries to get source interface from given interface name, and handle to it acquired from [get_interface_handle]
/// # Arguments
/// * `iface` - name of the interface to get, for example "VGUI_Panel009"
/// * `factory` - handle to the interface, acquired from [get_interface_handle]
/// # Examples
/// Getting the raw PaintTraverse function from vgui:
/// ```no_run
/// // Wrappers to these interfaces are already provided but they do not give raw function pointers which is needed to detour / modify the functions
/// // in any way, which you may want to do here, especially for painttraverse since you can safely run lua here if you queue it from a thread to avoid crashes.
/// use rglua::interface::{get_interface_handle, get_from_interface, IPanel};
/// type PaintTraverseFn = extern "fastcall" fn(&'static IPanel, usize, bool, bool);
/// let handle = unsafe { get_interface_handle("vgui2.dll").unwrap() };
///
/// let vgui_interface = get_from_interface("VGUI_Panel009", handle)
///     .unwrap() as *mut IPanel;
///
/// unsafe {
///     // Use as_ref to access fields of the interface
///     let panel_iface = vgui_interface
///         .as_ref() // Unsafe as Rust doesn't know whether the interface is really valid or not
///         .unwrap();
///
///
///     // Transmute the function address from the offset into our known signature
///     // You should use Interface.get_raw for this though
///     let paint_traverse: PaintTraverseFn = std::mem::transmute(
///         (panel_iface.vtable as *mut *mut std::ffi::c_void)
///             .offset(41) // vtable offset as seen in [interface/panel.rs]
///             .read()
///     );
/// }
/// ```
pub fn get_from_interface(
	iface: &str,
	factory: CreateInterfaceFn,
) -> Result<*mut (), InterfaceError> {
	let mut status = 0;

	let iface = try_cstr!(iface)?;

	let result = factory(iface.as_ptr(), &mut status);

	if status == 0 && !result.is_null() {
		Ok(result as *mut ())
	} else {
		Err(InterfaceError::FactoryNotFound)
	}
}
