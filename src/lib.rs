pub mod data;
pub mod models;
pub mod ui;
pub mod utils;

pub use ui::app::MyApp;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    // Better error messages in web console
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            canvas_id,
            web_options,
            Box::new(|cc| Box::new(MyApp::new(cc))),
        )
        .await
        .expect("Failed to start eframe");
    });

    Ok(())
}
