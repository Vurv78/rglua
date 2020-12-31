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

This is currently not in a very buildable state. Once it is, I will make the first release and add example modules to the repo.
If you want to look into how to build your projects, see my other repo https://github.com/Vurv78/gmod-rust-test and build.bat.
You need to build to 32 bit if you want to build to srcds, since it is 32 bit and you need to make sure your rust library is a "cdylib".
