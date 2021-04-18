// Types are in camelcase + C prefix
#![allow(non_snake_case)]

pub mod types;
pub mod globals;
pub mod helpers;

use types::*;
use globals::{
    LUA_GLOBALSINDEX
};

use std::path::{Path, PathBuf};
extern crate dlopen;

use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
pub struct LuaSharedInterface {
    // GMOD
    pub CreateInterface: extern fn(name: CharBuf, ret_code: CInt) -> *mut CVoid,

    // Runners
    pub luaL_loadbufferx: extern fn(state: LuaState, code: CharBuf, size: SizeT, id: CharBuf, mode: CharBuf) -> CInt,
    pub luaL_loadbuffer: extern fn(state: LuaState, code: CharBuf, size: SizeT, id: CharBuf) -> CInt,
    pub luaL_loadstring: extern fn(state: LuaState, code: CharBuf) -> CInt,
    pub lua_pcall: extern fn(state: LuaState, nargs: CInt, nresults: CInt, msgh: CInt) -> CInt,
    pub lua_call: extern fn(state: LuaState, nargs: CInt, nresults: CInt) -> CInt,
    pub lua_cpcall: extern fn(state: LuaState, func: LuaCFunction, userdata: *mut CVoid ),

    // Setters
    pub lua_setfield: extern fn(state: LuaState, idx: CInt, name: CharBuf),
    pub lua_setmetatable: extern fn(state: LuaState, idx: CInt),
    pub lua_settop: extern fn(state: LuaState, ind: CInt),
    pub lua_setupvalue: extern fn(state: LuaState, fidx: CInt, idx: CInt) -> CharBuf,
    pub lua_setfenv: extern fn(state: LuaState, idx: CInt) -> CInt,
    pub lua_settable: extern fn(state: LuaState, idx: CInt),

    // Getters
    pub lua_gettable: extern fn(state: LuaState, idx: CInt),
    pub lua_getfield: extern fn(state: LuaState, idx: CInt, key: CharBuf),
    pub lua_getupvalue: extern fn(state: LuaState, fidx: CInt, idx: CInt) -> CharBuf,
    pub lua_type: extern fn(state: LuaState, idx: CInt) -> CInt,

    // Getters (with "to")
    pub lua_tolstring: extern fn(state: LuaState, ind: CInt, size: SizeT) -> CharBuf,
    pub lua_toboolean: extern fn(state: LuaState, idx: CInt) -> CInt,
    pub lua_tocfunction: extern fn(state: LuaState, idx: CInt) -> LuaCFunction,

    // Push functions
    pub lua_pushstring: extern fn(state: LuaState, s: CharBuf),
    pub lua_pushlstring: extern fn(state: LuaState, s: CharBuf, sz: SizeT),
    pub lua_pushnil: extern fn(state: LuaState),
    pub lua_pushnumber: extern fn(state: LuaState, num: LuaNumber),
    pub lua_pushvalue: extern fn(state: LuaState, idx: CInt),
    pub lua_pushcclosure: extern fn(state: LuaState, fnc: LuaCFunction, idx: CInt),

    // Creation
    pub luaL_newstate: extern fn() -> LuaState,
    pub lua_createtable: extern fn(state: LuaState, narr: CInt, nrec: CInt),

    // Raise Errors
    pub luaL_typerror: extern fn(state: LuaState, narg: CInt, typename: CharBuf) -> CInt
}

// C++ Macros & Custom Functions
impl LuaSharedInterface {
    pub fn lua_pop(&self, state: LuaState, ind: CInt) {
        self.lua_settop(state, -(ind)-1);
    }

    pub fn lua_isboolean(&self, state: LuaState, ind: CInt) -> bool {
        self.lua_type(state, ind) == luatypes::BOOL
    }

    pub fn lua_setglobal(&self, state: LuaState, name: CharBuf) {
        self.lua_setfield(state, LUA_GLOBALSINDEX, name);
    }

    pub fn lua_getglobal(&self, state: LuaState, name: CharBuf) {
        self.lua_getfield(state, LUA_GLOBALSINDEX, name);
    }

    pub fn lua_pushcfunction(&self, state: LuaState, fnc: LuaCFunction) {
        self.lua_pushcclosure(state, fnc, 0);
    }
}

// Global Static Stuff

extern crate once_cell;
use once_cell::sync::Lazy;

// Keep separate in case needed by crates.
pub static GMOD_DIR: Lazy<PathBuf> = Lazy::new(|| {
    // Get the attached process. If you inject or run a binary module, will always GarrysMod directory.
    std::env::current_dir().expect("Couldn't get current running directory.") // D:\SteamLibrary\steamapps\common\GarrysMod for example.
});

// Let me know if there's a neater way to do this.
// Also if you need BIN_PATH back, try and re-implement it here.
// I don't know how i'd go about it without it being very messy and not checking whether lua_shared exists or not.
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let game_bin = Path::new(&*GMOD_DIR)
        .join("bin");

    if cfg!( target_arch = "x86_64" ) {
        // x64 Platform. srcds is always 32 bit so we don't have to try and check that here.
        let full = game_bin
            .join("win64")
            .join("lua_shared.dll");

        return match full.exists() {
            true => Some(full),
            false => None
        }
    } else {
        // x86 Platform
        let game_full = game_bin.join("lua_shared.dll");
        if game_full.exists() {
            return Some(game_full)
        } else {
            // Sometimes GarrysMod/garrysmod/bin contains lua_shared rather than just GarrysMod/bin.
            // I think it only happens on srcds hosted servers. So this is needed for binary modules on the sv side.
            let srcds_full = Path::new(&*GMOD_DIR)
                .join("garrysmod")
                .join("bin")
                .join("lua_shared.dll");
            return match srcds_full.exists() {
                true => Some(srcds_full),
                false => None
            }
        }
    }
});

pub static LUA_SHARED: Lazy< Container<LuaSharedInterface> > = Lazy::new(|| {
    let dll_path = match &*LUA_SHARED_PATH {
        Some(path) => path,
        None => panic!("Couldn't get lua_shared location. Make sure it's at GarrysMod/bin/ or GarrysMod/garrysmod/bin/")
    };

    return match unsafe {Container::load(dll_path)} {
        Ok(lib) => lib,
        Err(why) => panic!("Path DLL tried to load: {}, Error Reason: {}. Report this on github.", dll_path.display(), why)
    }
});