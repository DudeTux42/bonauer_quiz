mod models;
mod ui;
mod utils;
mod error;
mod data;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Bonauer Quiz GUI",
        options,
        Box::new(|_cc| Ok(Box::new(ui::MyApp::default()))),
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    eframe::start_web(
        "bonauer_quiz_canvas", // id of the canvas element
        Box::new(|_cc| Box::new(ui::MyApp::default()))),
    )?;
    Ok(())
}
