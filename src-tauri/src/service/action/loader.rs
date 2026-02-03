#[hot_lib_reloader::hot_module(dylib = "web")]
mod hot_lib {
    pub use lib::State;
    hot_functions_from_file!("lib/src/lib.rs");
}
