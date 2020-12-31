#![no_std]
fn main() {
    cc::Build::new()
        .cpp(true) // Compile C++
        .file("glua-headers/init.cpp")
        .compile("gluainit.a");
}