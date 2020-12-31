# rglua

This is a Rust library that creates bindings for garrysmod binary module creation.
More information on binary modules: https://wiki.facepunch.com/gmod/Creating_Binary_Modules

Add this to your Cargo.toml
```toml

[lib]
crate-type = ["cdylib"] # This tells rust we want to create a .dll file that links to C code.

[dependencies]
rglua = { git = "https://github.com/Vurv78/rglua", branch = "main" } # This gives you all of the good stuff w/ bindings

```

Example Module:
```rust
// Todo
```
