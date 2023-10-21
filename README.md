# Steps

Create rust library skel:
```cargo new --lib rust-lib```

In order to build:
```cargo build <-r>``` -r or --release for the release build

In order to generate C/C++ bindings:
```cargo install --force cbindgen```

In order to generate bindings for C/C++ targets to Rust
```cargo install --force bindgen```

In order to generate header for the library:
```cbindgen --config cbindgen.toml --crate rust-lib --output rust_lib.h```