#[cfg(not(target_arch = "wasm32"))]
fn main() {
    bonauer_quiz_lib::main();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Empty main function for WASM builds
    // The actual entry point is the start() function in lib.rs
}
