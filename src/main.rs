use bonauer_quiz_lib::ui::app::MyApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Bonauer Quiz",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}
