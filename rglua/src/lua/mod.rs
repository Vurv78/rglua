use libloading::Library;
use std::path::PathBuf;

use once_cell::sync::Lazy;

mod shared;
pub use shared::*;

mod globals;
pub use globals::*;

pub mod types;
pub use types::*;

// Keep separate in case needed by crates.
pub static GMOD_DIR: Lazy<PathBuf> = Lazy::new(|| {
	// Get the attached process. If you inject or run a binary module, will always GarrysMod directory. If not then you did something wrong.
	std::env::current_dir().expect("Couldn't get current_dir.")
});

// Let me know if there's a neater way to do this.
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let mut full: PathBuf;

	if cfg!(target_arch = "x86_64") {
		// x86_64 Platform. Always should be in GMOD/bin/win64
		full = GMOD_DIR.join("bin").join("win64").join("lua_shared.dll");
	} else {
		// x86 Platform, Either in GMOD/garrysmod/bin or GMOD/bin
		full = GMOD_DIR
			.join("garrysmod")
			.join("bin")
			.join("lua_shared.dll");
		if !full.exists() {
			full = GMOD_DIR.join("bin").join("lua_shared.dll");
		}
	}
	if !full.exists() {
		eprintln!(
			"lua_shared.dll couldn't be found! Tried to get {}",
			full.display()
		);
		return None;
	}

	Some(full)
});

pub static LUA_SHARED_RAW: Lazy<Library> = Lazy::new(|| {
	let path = LUA_SHARED_PATH
		.as_ref()
		.expect("Couldn't find lua_shared.dll!");
	unsafe { Library::new(path).expect("Could not open library") }
});
