# rglua [![Release Shield](https://img.shields.io/github/v/release/Vurv78/rglua)](https://github.com/Vurv78/rglua/releases/latest) ![Linux Build Status](https://www.travis-ci.com/Vurv78/rglua.svg?branch=main) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://img.shields.io/discord/824727565948157963?color=7289DA&label=chat&logo=discord)](https://discord.gg/epJFC6cNsw)

This is a crate that contains bindings for using the lua c api in garrysmod through bindings using rust-dlopen.
Can be used for either binary modules or just manual injections into gmod, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

This works by finding a ``lua_shared.dll`` file relative to the currently running program, so you need to make sure your file is either in ``GarrysMod/bin/`` or ``GarrysMod/garrysmod/bin`` for srcds servers. The library will panic if the file is not found.

More information on binary modules can be found on the garrysmod wiki: [Creating Binary Modules](https://wiki.facepunch.com/gmod/Creating_Binary_Modules) and an example can be found at the bottom of this file.

## Usage

Add this to your ``Cargo.toml`` file
```toml
[lib]
crate-type = ["cdylib"] # This tells rust we want to create a .dll file that links to C code.

[dependencies]
rglua = { git = "https://github.com/Vurv78/rglua" }
```

## Building
After [installing rust, ](https://www.rust-lang.org/tools/install) just run  ``cargo build --release``.

If you are building to 32 bit for srcds or non x64 garrysmod, first do:  
``rustup target add i686-pc-windows-msvc`` in order to make rust download any 32 bit libraries needed to compile this.  

And finally run:  
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
	types::LuaState,
	cstring,
	lua_shared::*,
	lua_getglobal,
};

#[no_mangle]
pub extern fn gmod13_open(state: LuaState) -> i32 {
	lua_getglobal!( state, cstring!("print") );
	lua_pushstring( state, cstring!("Hello from rust!") );
	lua_call( state, 1, 0 );
	0
}

#[no_mangle]
pub extern fn gmod13_close(_state: LuaState) -> i32 {
	0
}
```

## Acknowledgements
### [garrysmod_common](https://github.com/danielga/garrysmod_common)
This is heavily based off of garrysmod_common, in how we export the lua_shared functions and trying to replicate everything from the Lua C Api.

### [rust-dlopen](https://github.com/szymonwieloch/rust-dlopen)
Thanks to this library we're able to make rglua cross platform.
