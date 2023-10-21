cmake_minimum_required(VERSION 3.20)
project(cpp-lib-rust-call)

list(PREPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(get_corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/rust_lib/Cargo.toml)

include_directories(cbindgen_headers)
add_subdirectory(cpp_call_rust)
