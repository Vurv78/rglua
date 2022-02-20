use libloading::Library;
use std::path::PathBuf;

use once_cell::sync::Lazy;

mod shared;
pub use shared::*;

mod globals;
pub use globals::*;

pub mod types;
pub use types::*;

/// Path to lua_shared dynamic library, found relative to [std::env::current_dir]
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> =
	Lazy::new(|| Some(PathBuf::from("bin/win64/lua_shared.dll")));

#[cfg(all(target_os = "windows", target_arch = "x86"))]
/// Path to lua_shared dynamic library, found relative to [std::env::current_dir]
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let gmod = std::env::current_dir().expect("Failed to get current dir");

	for path in ["garrysmod/bin/lua_shared.dll", "bin/lua_shared.dll"] {
		let full = gmod.join(path);
		if full.exists() {
			return Some(full);
		}
	}
	None
});

#[cfg(all(target_os = "macos"))]
/// Path to lua_shared dynamic library, found relative to [std::env::current_dir]
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> =
	Lazy::new(|| Some(PathBuf::from("garrysmod/bin/lua_shared.dylib")));

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let gmod = std::env::current_dir().expect("Failed to get current dir");

	for path in [
		"garrysmod/bin/lua_shared_srv.so",
		"garrysmod/bin/lua_shared.so",
		"bin/linux32/lua_shared.so",
		"bin/linux32/lua_shared_client.so"
	] {
		let full = gmod.join(path);
		if full.exists() {
			return Some(full);
		}
	}
	None
});

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
/// Path to lua_shared dynamic library, found relative to [std::env::current_dir]
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let gmod = std::env::current_dir().expect("Failed to get current dir");

	for path in [
		"bin/linux64/lua_shared.so",
		"bin/linux64/lua_shared_client.so"
	] {
		let full = gmod.join(path);
		if full.exists() {
			return Some(full);
		}
	}
	None
});

/// This tries to retrieve [LUA_SHARED_PATH], creates a [libloading::Library] to it and returns it.
/// If it could not find lua_shared.dll or create a [libloading::Library], this will panic!
pub static LUA_SHARED_RAW: Lazy<Library> = Lazy::new(|| {
	let path = LUA_SHARED_PATH
		.as_ref()
		.expect("Couldn't find lua_shared dylib!");
	unsafe { Library::new(path).expect("Could not open library") }
});
