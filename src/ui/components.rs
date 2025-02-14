use eframe::egui;

/// A reusable button for quiz options.
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
        ui.add(
            egui::Button::new(egui::RichText::new(self.text).size(self.size))
                .fill(self.background)
                .min_size(egui::Vec2::new(ui.available_width(), 40.0)),
        )
    }
}

/// Displays centered text with a given style.
pub fn centered_text(ui: &mut egui::Ui, text: &str, size: f32) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(text).size(size).strong());
    });
}

/// Displays a quiz header with a main title and an optional subtitle.
pub fn quiz_header(ui: &mut egui::Ui, title: &str, subtitle: Option<&str>, title_size: f32) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new(title).size(title_size));
        if let Some(sub) = subtitle {
            ui.label(egui::RichText::new(sub).size(title_size * 0.75));
        }
    });
}

/// Displays the current score aligned to the top right.
pub fn score_display(ui: &mut egui::Ui, score: usize) {
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - 100.0);
        ui.label(format!("Score: {}", score));
    });
}

/// A button specifically for answer options in a quiz.
/// It allows specifying different colors for default, correct, and incorrect states.
pub struct AnswerOptionButton<'a> {
    text: &'a str,
    size: f32,
    default_color: egui::Color32,
    correct_color: egui::Color32,
    incorrect_color: egui::Color32,
}

impl<'a> AnswerOptionButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            size: 20.0,
            default_color: egui::Color32::from_rgb(40, 40, 40),
            correct_color: egui::Color32::from_rgb(100, 255, 100),
            incorrect_color: egui::Color32::from_rgb(255, 100, 100),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn default_color(mut self, color: egui::Color32) -> Self {
        self.default_color = color;
        self
    }

    pub fn correct_color(mut self, color: egui::Color32) -> Self {
        self.correct_color = color;
        self
    }

    pub fn incorrect_color(mut self, color: egui::Color32) -> Self {
        self.incorrect_color = color;
        self
    }

    /// Shows the button using the specified state.
    /// `state` can be "default", "correct", or "incorrect" to change the color.
    pub fn show(self, ui: &mut egui::Ui, state: &str) -> egui::Response {
        let background = match state {
            "correct" => self.correct_color,
            "incorrect" => self.incorrect_color,
            _ => self.default_color,
        };

        ui.add(
            egui::Button::new(egui::RichText::new(self.text).size(self.size))
                .fill(background)
                .min_size(egui::Vec2::new(ui.available_width(), 40.0)),
        )
    }
}
