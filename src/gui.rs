use eframe::egui;

#[derive(Default)]
pub struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bonauer Quiz");
            if ui.button("Start Quiz").clicked() {
                // Handle button click
                println!("Quiz has started!");
            }
        });
    }
}
