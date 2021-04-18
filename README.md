# rglua [![Release Shield](https://img.shields.io/github/v/release/Vurv78/rglua)](https://github.com/Vurv78/rglua/releases/latest) ![Linux Build Status](https://www.travis-ci.com/Vurv78/rglua.svg?branch=main) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://discordapp.com/api/guilds/824727565948157963/widget.png)](https://discord.gg/epJFC6cNsw)

This is a crate that contains bindings for using the lua c api in garrysmod through bindings using rust-dlopen.
Can be used for either binary modules or just manual injections into gmod, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

This works by finding a ``lua_shared.dll`` file relative to the currently running program, so you need to make sure your file is either in ``GarrysMod/bin/`` or ``GarrysMod/garrysmod/bin`` for srcds servers. The library will panic if the file is not found otherwise until it is eventually needed.

More information on binary modules can be found on the garrysmod wiki: [Creating Binary Modules](https://wiki.facepunch.com/gmod/Creating_Binary_Modules) and an example can be found at the bottom of this file.

## Usage

Add this to your ``Cargo.toml`` file
```toml
[lib]
crate-type = ["cdylib"] # This tells rust we want to create a .dll file that links to C code.

[dependencies]
rglua = { git = "https://github.com/Vurv78/rglua", branch = "main" }
```

## Building
Make sure you build to 32 bit if you want to use the module with srcds / on a local server.
``cargo build --release --target=i686-pc-windows-msvc``

Also do this if you have never compiled to 32 bit, to get rustup to install 32 bit versions of everything you need
``rustup target add i686-pc-windows-msvc``

## Notes
*  I have never tested this outside of Windows and won't.
   If there are any issues on other platforms, I will gladly accept any PRs you may make but I won't be able to help you myself.

* The nature of this crate is super unsafe and sort of defeats the purpose of rust's safety because of the interfacing you require to unsafe C code and the nature of linking to them.

## Example Module
```rust
use rglua::{
    types::{
        LuaState
    },
    cstring,
    LUA_SHARED
};

#[no_mangle]
pub extern fn gmod13_open(state: LuaState) -> i32 {
    let shared = &*LUA_SHARED;
    shared.lua_getglobal(state, cstring!("print") );
    shared.lua_pushstring(state, cstring!("Hello from rust!") );
    shared.lua_call(state, 1, 0);
    0
}

#[no_mangle]
pub extern fn gmod13_close(_state: LuaState) -> i32 {
    0
}
```
