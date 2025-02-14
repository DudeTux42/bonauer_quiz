use eframe::egui;

/// A reusable button for quiz options.
pub struct QuizButton<'a> {
    text: &'a str,             // The text that will appear on the button
    size: f32,                 // The size of the text on the button
    background: egui::Color32, // Background color of the button
}

impl<'a> QuizButton<'a> {
    /// Creates a new `QuizButton` with the given text.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,                                            // Set the button text
            size: 20.0,                                      // Default size for text
            background: egui::Color32::from_rgb(40, 40, 40), // Default background color
        }
    }

    /// Sets the size of the button text.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size; // Update the text size
        self
    }

    /// Sets the background color of the button.
    pub fn background(mut self, color: egui::Color32) -> Self {
        self.background = color; // Update the background color
        self
    }

    /// Displays the button on the UI with the current properties.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        // Create a button with the text, size, and background color defined.
        ui.add(
            egui::Button::new(egui::RichText::new(self.text).size(self.size)) // Button with rich text and size
                .fill(self.background) // Set the background color
                .min_size(egui::Vec2::new(ui.available_width(), 40.0)), // Minimum button size
        )
    }
}

/// Displays centered text with a given style.
pub fn centered_text(ui: &mut egui::Ui, text: &str, size: f32) {
    // Use vertical centering to position the text in the middle of the UI.
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(text).size(size).strong()); // Display the text with the given size and bolded
    });
}

/// Displays a quiz header with a main title and an optional subtitle.
pub fn quiz_header(ui: &mut egui::Ui, title: &str, subtitle: Option<&str>, title_size: f32) {
    ui.horizontal(|ui| {
        // Display the title in a heading style with the specified font size.
        ui.heading(egui::RichText::new(title).size(title_size));
        // If a subtitle is provided, display it with a smaller font size.
        if let Some(sub) = subtitle {
            ui.label(egui::RichText::new(sub).size(title_size * 0.75)); // Subtitle size smaller than title
        }
    });
}

/// Displays the current score aligned to the top right.
pub fn score_display(ui: &mut egui::Ui, score: usize) {
    ui.horizontal(|ui| {
        // Leave space on the left side so the score is aligned to the right.
        ui.add_space(ui.available_width() - 100.0);
        // Display the score label.
        ui.label(format!("Score: {}", score));
    });
}

/// A button specifically for answer options in a quiz.
/// It allows specifying different colors for default, correct, and incorrect states.
pub struct AnswerOptionButton<'a> {
    text: &'a str,                  // Text to display on the button
    size: f32,                      // Size of the button text
    default_color: egui::Color32,   // Color of the button in default state
    correct_color: egui::Color32,   // Color of the button when the answer is correct
    incorrect_color: egui::Color32, // Color of the button when the answer is incorrect
}

impl<'a> AnswerOptionButton<'a> {
    /// Creates a new `AnswerOptionButton` with the given text.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,                                                    // Set the button text
            size: 20.0,                                              // Default text size
            default_color: egui::Color32::from_rgb(40, 40, 40),      // Default button color
            correct_color: egui::Color32::from_rgb(100, 255, 100), // Color when answer is correct (green)
            incorrect_color: egui::Color32::from_rgb(255, 100, 100), // Color when answer is incorrect (red)
        }
    }

    /// Sets the size of the button text.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size; // Update the button text size
        self
    }

    /// Sets the default color of the button.
    pub fn default_color(mut self, color: egui::Color32) -> Self {
        self.default_color = color; // Set the default button color
        self
    }

    /// Sets the color of the button when the answer is correct.
    pub fn correct_color(mut self, color: egui::Color32) -> Self {
        self.correct_color = color; // Set the correct answer button color
        self
    }

    /// Sets the color of the button when the answer is incorrect.
    pub fn incorrect_color(mut self, color: egui::Color32) -> Self {
        self.incorrect_color = color; // Set the incorrect answer button color
        self
    }

    /// Shows the button using the specified state.
    /// `state` can be "default", "correct", or "incorrect" to change the color.
    pub fn show(self, ui: &mut egui::Ui, state: &str) -> egui::Response {
        // Choose the button's background color based on the state
        let background = match state {
            "correct" => self.correct_color,     // Correct answer: green
            "incorrect" => self.incorrect_color, // Incorrect answer: red
            _ => self.default_color,             // Default state: dark gray
        };

        // Display the button with the selected background color and other properties
        ui.add(
            egui::Button::new(egui::RichText::new(self.text).size(self.size)) // Button with text and size
                .fill(background) // Set the background color
                .min_size(egui::Vec2::new(ui.available_width(), 40.0)), // Minimum button size
        )
    }
}
