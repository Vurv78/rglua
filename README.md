# 🌑 ``rglua`` [![cratesio](https://img.shields.io/crates/v/rglua.svg)](https://crates.io/crates/rglua) ![Build Status](https://github.com/Vurv78/rglua/actions/workflows/ci.yml/badge.svg) [![License](https://img.shields.io/github/license/Vurv78/rglua?color=red)](https://opensource.org/licenses/Apache-2.0) [![github/Vurv78](https://img.shields.io/discord/824727565948157963?label=Discord&logo=discord&logoColor=ffffff&labelColor=7289DA&color=2c2f33)](https://discord.gg/epJFC6cNsw)

This is a crate that allows interop with the (g)luajit c api as well as the source sdk through libloading and vtable bindings.
You can then use these for binary modules or manually injected code, like with [Autorun-rs](https://github.com/Vurv78/Autorun-rs)

More information on binary modules can be found on the garrysmod wiki: [Creating Binary Modules](https://wiki.facepunch.com/gmod/Creating_Binary_Modules) and examples [can be found here.](https://github.com/Vurv78/rglua/tree/master/examples)
## Usage
If you are targeting 32 bit make sure to install the toolchain and build to it:
```bash
rustup target add i686-pc-windows-msvc
cargo build --target=i686-pc-windows-msvc
```

## Comparison
There are actually a decent amount of libraries out there for gmod development.
Here's a comparison and why you could use this one.

[rglua]: https://crates.io/crates/rglua
[rust-glua-sys]: https://github.com/SpiralP/rust-glua-sys
[gmod-rs]: https://crates.io/crates/gmod
[gmrs]: https://github.com/diogo464/gmrs

| Library                           | [rglua] | [rust-glua-sys] | [gmod-rs]   | [gmrs] |
|-----------------------------------|---------|-----------------|-------------|--------|
| *Full* Lua C Api Bindings         | ✔️     | ❌              | ❌         | ❌    |
| On Crates.io                      | ✔️     | ❌              | ✔️         | ❌    |
| Proc Macros                       | ✔️     | ❌              | ✔️         | ✔️    |
| Interfacing w/  Source SDK        | ✔️     | ❌              | ❌         | ❌    |
| Returning Result<> from functions | ✔️	 | ❌              | ❌         | ✔️    |
| Can be used on stable             | ✔️     | ✔️              | ❌         | ✔️    |
| Real world examples               | ✔️     | ❌              | 〰️         | ✔️    |
| Linux / OSX Support               | ✔️     | ❌              | ✔️         | ✔️    |
| Github Stars                      | 😢     | 👍              | 👑         | 🤷‍♂️    |

__*You can help with that last one 😉*__

## Acknowledgements
### [garrysmod_common](https://github.com/danielga/garrysmod_common)
This is heavily based off of garrysmod_common, in how we export the lua_shared functions and trying to replicate everything from the Lua C Api.
