use crate::ui::app::MyApp;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bonauer Quiz",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
    .expect("Failed to start application");
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    eframe::start_web(
        canvas_id,
        web_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

// Add this as well for including the app module
pub mod data;
pub mod models;
pub mod ui;
pub mod utils;
