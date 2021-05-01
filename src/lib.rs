// Types are in camelcase + C prefix
#![allow(non_snake_case)]

pub mod types;
pub mod globals;
pub mod helpers;

use types::*;
use globals::Lua;

use std::path::PathBuf;
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
    pub lua_cpcall: extern fn(state: LuaState, func: LuaCFunction, userdata: *mut CVoid ) -> CInt,
    pub luaL_callmeta: extern fn(state: LuaState, obj: CInt, name: CharBuf) -> CInt,

    // Setters
    pub lua_setfield: extern fn(state: LuaState, idx: CInt, name: CharBuf),
    pub lua_setmetatable: extern fn(state: LuaState, idx: CInt),
    pub lua_settop: extern fn(state: LuaState, ind: CInt),
    pub lua_setupvalue: extern fn(state: LuaState, fidx: CInt, idx: CInt) -> CharBuf,
    pub lua_setfenv: extern fn(state: LuaState, idx: CInt) -> CInt,
    pub lua_settable: extern fn(state: LuaState, idx: CInt),
    pub lua_rawset: extern fn(state: LuaState, idx: CInt), // lua_settable but no metamethods called
    pub lua_rawseti: extern fn(state: LuaState, idx: CInt, n: CInt), // t[n] = v

    // Getters
    pub lua_gettable: extern fn(state: LuaState, idx: CInt),
    pub lua_rawget: extern fn(state: LuaState, idx: CInt), // lua_gettable but no metamethods called
    pub lua_rawgeti: extern fn(state: LuaState, idx: CInt, n: CInt), // lua_gettable but no metamethods called

    pub lua_getfield: extern fn(state: LuaState, idx: CInt, key: CharBuf),
    pub lua_getupvalue: extern fn(state: LuaState, fidx: CInt, idx: CInt) -> CharBuf,
    pub lua_type: extern fn(state: LuaState, idx: CInt) -> CInt,
    pub lua_typename: extern fn(state: LuaState, typeid: CInt) -> CharBuf, // To be used with the return value of lua_type

    // Getters (with "to")
    pub lua_tolstring: extern fn(state: LuaState, ind: CInt, size: SizeT) -> CharBuf,
    pub lua_toboolean: extern fn(state: LuaState, idx: CInt) -> CInt,
    pub lua_tocfunction: extern fn(state: LuaState, idx: CInt) -> LuaCFunction,
    pub lua_tointeger: extern fn(state: LuaState, idx: CInt) -> LuaInteger,
    pub lua_tonumber: extern fn(state: LuaState, idx: CInt) -> LuaNumber,
    pub lua_topointer: extern fn(state: LuaState, idx: CInt) -> *mut CVoid,
    pub lua_tothread: extern fn(state: LuaState, idx: CInt) -> LuaState,
    pub lua_touserdata: extern fn(state: LuaState, idx: CInt) -> *mut CVoid,

    // Push functions
    pub lua_pushstring: extern fn(state: LuaState, s: CharBuf),
    pub lua_pushlstring: extern fn(state: LuaState, s: CharBuf, sz: SizeT),
    pub lua_pushnil: extern fn(state: LuaState),
    pub lua_pushnumber: extern fn(state: LuaState, num: LuaNumber),
    pub lua_pushvalue: extern fn(state: LuaState, idx: CInt),
    pub lua_pushcclosure: extern fn(state: LuaState, fnc: LuaCFunction, idx: CInt),

    // Type Checks
    pub luaL_checkinteger: extern fn(state: LuaState, narg: CInt) -> LuaInteger,
    pub luaL_checknumber: extern fn(state: LuaState, narg: CInt) -> LuaNumber,
    pub luaL_checklstring: extern fn(state: LuaState, narg: CInt) -> CharBuf,

    // Type Checks that return nothing
    pub luaL_checkstack: extern fn(state: LuaState, size: CInt, msg: CharBuf),
    pub luaL_checkany: extern fn(state: LuaState, narg: CInt),
    pub luaL_checktype: extern fn(state: LuaState, narg: CInt, typeid: CInt),
    pub luaL_checkudata: extern fn(state: LuaState, narg: CInt, len: SizeT),

    // Creation
    pub luaL_newstate: extern fn() -> LuaState,
    pub lua_createtable: extern fn(state: LuaState, narr: CInt, nrec: CInt),

    // Destruction
    pub lua_close: extern fn(state: LuaState), // Destroys the lua state

    // JIT
    // Returns 1 for success, 0 for failure
    pub luaJIT_setmode: extern fn(state: LuaState, idx: CInt, jit_mode: CInt) -> CInt,

    // Coroutines
    pub lua_yield: extern fn(state: LuaState, nresults: CInt) -> CInt,
    pub lua_status: extern fn(state: LuaState) -> CInt,
    pub lua_resume_real: extern fn(state: LuaState, narg: CInt) -> CInt,

    // Comparison
    pub lua_equal: extern fn(state: LuaState, ind1: CInt, ind2: CInt) -> CInt, // Returns 1 or 0 bool
    pub lua_rawequal: extern fn(state: LuaState, ind1: CInt, ind2: CInt) -> CInt,

    // Raising Errors
    pub luaL_typerror: extern fn(state: LuaState, narg: CInt, typename: CharBuf) -> CInt,
}

// C++ Macros & Custom Functions
impl LuaSharedInterface {
    pub fn lua_pop(&self, state: LuaState, ind: CInt) {
        self.lua_settop(state, -(ind)-1);
    }

    pub fn lua_isboolean(&self, state: LuaState, ind: CInt) -> bool {
        self.lua_type(state, ind) == Lua::Type::Bool as i32
    }

    pub fn lua_setglobal(&self, state: LuaState, name: CharBuf) {
        self.lua_setfield(state, Lua::GLOBALSINDEX, name);
    }

    pub fn lua_getglobal(&self, state: LuaState, name: CharBuf) {
        self.lua_getfield(state, Lua::GLOBALSINDEX, name);
    }

    pub fn lua_pushcfunction(&self, state: LuaState, fnc: LuaCFunction) {
        self.lua_pushcclosure(state, fnc, 0);
    }

    pub fn lua_tostring(&self, state: LuaState, idx: CInt) -> CharBuf {
        self.lua_tolstring(state, idx, 0)
    }
    pub fn lua_resume(&self, state: LuaState, narg: CInt) -> CInt {
        self.lua_resume_real(state, narg)
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

/// Where the lua_shared file binary is.
pub static BIN_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let gm_dir = &*GMOD_DIR;
    match gm_dir.join("bin") {
        bin if bin.exists() && bin.join("lua_shared.dll").exists() => bin, // GarrysMod/bin
        _ => {
            let garrysmod_bin = gm_dir.join("garrysmod").join("bin");
            if !garrysmod_bin.exists() {
                panic!("Couldn't find a bin folder in GarrysMod/bin or GarrysMod/garrysmod/bin.");
            }
            garrysmod_bin // GarrysMod/garrysmod/bin
        },
    }
});

// Let me know if there's a neater way to do this.
// Also if you need BIN_PATH back, try and re-implement it here.
// I don't know how i'd go about it without it being very messy and not checking whether lua_shared exists or not.
pub static LUA_SHARED_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let game_bin = &*BIN_DIR;

    if cfg!( target_arch = "x86_64" ) {
        // x64 Platform. srcds is always 32 bit so we don't have to try and check that here.
        let full = game_bin
            .join("win64")
            .join("lua_shared.dll");

        return match full.exists() {
            true => Some(full),
            false => {
                eprintln!("x64, {}", full.display());
                None
            }
        }
    } else {
        // x86 Platform
        let game_full = game_bin.join("lua_shared.dll");
        return match game_full.exists() {
            true => Some(game_full),
            false => {
                eprintln!("game_full, {}", game_full.display());
                None
            }
        }
    }
});

type LuaSharedLibrary = Container<LuaSharedInterface>;

// Returns the underlying result of trying to create a luasharedinterface.
// Useful if you don't want your program to panic at all.
pub static LUA_SHAREDR: Lazy< Result< LuaSharedLibrary, dlopen::Error> > = Lazy::new(|| {
    let dll_path = match &*LUA_SHARED_PATH {
        Some(path) => path,
        None => panic!("Couldn't get lua_shared location. Make sure it's at GarrysMod/bin/ or GarrysMod/garrysmod/bin/")
    };
    unsafe {Container::load(dll_path)}
});

pub static LUA_SHARED: Lazy< &LuaSharedLibrary > = Lazy::new(|| {
    match &*LUA_SHAREDR {
        Ok(lib) => lib,  // We shouldn't need a mutable LuaSharedLibrary.
        Err(why) => panic!("Couldn't load lua_shared.dll. Error Reason: {}. Report this on github.", why)
    }
});