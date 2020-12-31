# rglua

This is a Rust library that creates bindings for garrysmod binary module creation.
More information on binary modules: https://wiki.facepunch.com/gmod/Creating_Binary_Modules

## How to build

Add this to your Cargo.toml
```toml

[lib]
crate-type = ["cdylib"] # This tells rust we want to create a .dll file that links to C code.

[dependencies]
rglua = { git = "https://github.com/Vurv78/rglua", branch = "main" } # This gives you all of the good stuff w/ bindings

```

Make sure you build to 32 bit if you want to use the module with srcds / on a local server.
``cargo build --release --target=i686-pc-windows-msvc``

Also do
``rustup add target i686-pc-windows-msvc``

More Info: https://github.com/Vurv78/gmod-rust-test

## Example Module
```rust
use rglua::{RLuaState,LuaState};

#[no_mangle]
pub extern fn gmod13_open(state: LuaState) -> i32 {
    let mut wrapped = RLuaState::new(state);
    // This is the same as doing 'printgm!(wrapped,"Hello from rust!")'
    wrapped.get_global(&"print");
    wrapped.push_string(&"Hello from rust!");
    wrapped.call(1,0);

    0
}

#[no_mangle]
pub extern fn gmod13_close(state: LuaState) -> i32 {
    let _wrapped = RLuaState::new(state);
    0
}
```
