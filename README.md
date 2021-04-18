# rglua ![TravisCI Status](https://www.travis-ci.com/Vurv78/rglua.svg?branch=main) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://discordapp.com/api/guilds/824727565948157963/widget.png)](https://discord.gg/epJFC6cNsw)


This is a Rust library that contains bindings for tinkering with the lua c api in garrysmod through bindings using rust-dlopen.
Can be used for either binary modules or just manual injections into gmod, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

More information on binary modules: https://wiki.facepunch.com/gmod/Creating_Binary_Modules

An example module can be found below.

## How to build

Add this to your Cargo.toml.
```toml
[lib]
crate-type = ["cdylib"] # This tells rust we want to create a .dll file that links to C code.

[dependencies]
rglua = { git = "https://github.com/Vurv78/rglua", branch = "main" }
```

Make sure you build to 32 bit if you want to use the module with srcds / on a local server.
``cargo build --release --target=i686-pc-windows-msvc``

Also do this if you have never compiled to 32 bit, to get rustup to install 32 bit versions of everything you need
``rustup target add i686-pc-windows-msvc``

## Notes
The nature of this crate is super unsafe.  
Using rust sort of defeats the purpose because of the sheer amount of times you'll have to convert strings from and to C, and call lua c api functions.

Also, I have never tested this outside of Windows and won't. If there are any issues on other platforms, I will gladly accept any PRs you may make but I won't be able to help you myself.

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
