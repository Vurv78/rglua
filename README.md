# 🌑 ``rglua`` [![cratesio](https://img.shields.io/crates/v/rglua.svg)](https://crates.io/crates/rglua) ![Build Status](https://www.travis-ci.com/Vurv78/rglua.svg?branch=main) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://img.shields.io/discord/824727565948157963?label=Discord&logo=discord&logoColor=ffffff&labelColor=7289DA&color=2c2f33)](https://discord.gg/epJFC6cNsw)

This is a crate that contains bindings for using the lua c api in garrysmod through bindings using the rust libloading library.
Can be used for either binary modules or just manual injections into gmod, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

This works by finding a ``lua_shared.dll`` file relative to the currently running program, so you need to make sure your file is either in ``GarrysMod/bin/`` or ``GarrysMod/garrysmod/bin`` for srcds servers. The library will panic if the file is not found.

More information on binary modules can be found on the garrysmod wiki: [Creating Binary Modules](https://wiki.facepunch.com/gmod/Creating_Binary_Modules) and an example [can be found here.](https://github.com/Vurv78/rglua/tree/master/examples/is_even)
## Usage
If you are targeting 32 bit make sure to install the toolchain and build to it:
```bash
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc
```
## Acknowledgements
### [garrysmod_common](https://github.com/danielga/garrysmod_common)
This is heavily based off of garrysmod_common, in how we export the lua_shared functions and trying to replicate everything from the Lua C Api.
