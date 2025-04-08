pub mod data;
pub mod models;
pub mod ui;
pub mod utils;

// Native app entry point
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bonauer Quiz",
        native_options,
        Box::new(|cc| Box::new(ui::app::MyApp::new(cc))),
    )
    .expect("Failed to start application");
}

// Web app entry point
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set up panic hook
    console_error_panic_hook::set_once();

    // Initialize the app
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas", // ID of canvas element
                web_options,
                Box::new(|cc| Box::new(ui::app::MyApp::new(cc))),
            )
            .await
            .expect("Failed to start eframe");
    });

    Ok(())
}
