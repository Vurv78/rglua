# 🌑 ``rglua`` [![cratesio](https://img.shields.io/crates/v/rglua.svg)](https://crates.io/crates/rglua) ![Build Status](https://www.travis-ci.com/Vurv78/rglua.svg?branch=main) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://img.shields.io/discord/824727565948157963?label=Discord&logo=discord&logoColor=ffffff&labelColor=7289DA&color=2c2f33)](https://discord.gg/epJFC6cNsw)

This is a crate that allows interop with the luajit c api as well as the source sdk through libloading and vtable bindings.
You can then use these for binary modules or manually injected code, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

More information on binary modules can be found on the garrysmod wiki: [Creating Binary Modules](https://wiki.facepunch.com/gmod/Creating_Binary_Modules) and examples [can be found here.](https://github.com/Vurv78/rglua/tree/master/examples)
## Usage
If you are targeting 32 bit make sure to install the toolchain and build to it:
```bash
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc
```
## Acknowledgements
### [garrysmod_common](https://github.com/danielga/garrysmod_common)
This is heavily based off of garrysmod_common, in how we export the lua_shared functions and trying to replicate everything from the Lua C Api.
