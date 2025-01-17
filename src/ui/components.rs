use eframe::egui;

pub struct QuizButton<'a> {
    text: &'a str,
    size: f32,
    background: egui::Color32,
}

impl<'a> QuizButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            size: 20.0,
            background: egui::Color32::from_rgb(40, 40, 40),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn background(mut self, color: egui::Color32) -> Self {
        self.background = color;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::Button::new(
            egui::RichText::new(self.text).size(self.size),
        )
        .fill(self.background)
        .min_size(egui::Vec2::new(ui.available_width(), 40.0)))
    }
}

pub fn centered_text(ui: &mut egui::Ui, text: &str, size: f32) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(text)
                .size(size)
                .strong(),
        );
    });
}
