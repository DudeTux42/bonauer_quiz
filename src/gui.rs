use eframe::egui;
use crate::data::create_sample_quiz;
use crate::category::Category;
use crate::question::Question;
use crate::quiz::Quiz;
use std::time::{Duration, Instant};

pub struct MyApp {
    categories: Vec<Category>,
    selected_category: Option<Category>,
    current_questions: Vec<Question>,
    current_question_index: usize,
    score: usize,
    quiz: Quiz,
    last_guess_time: Option<Instant>, // Tracks when the user made a guess
}

impl Default for MyApp {
    fn default() -> Self {
        let quiz = create_sample_quiz();
        Self {
            categories: quiz.categories.clone().into_values().collect(),
            selected_category: None,
            current_questions: Vec::new(),
            current_question_index: 0,
            score: 0,
            quiz,
            last_guess_time: None, // Initialize with None
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let selected_category = self.selected_category.clone();
            if let Some(category) = selected_category {
                self.show_quiz(ui, &category);
            } else {
                self.show_main_menu(ui);
            }
        });
    }
}

impl MyApp {
    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bonauer Quiz");
        ui.label("Select a game mode:");

        for category in &self.categories {
            if ui.button(&category.name).clicked() {
                self.selected_category = Some(category.clone());
                self.current_questions = self.quiz.initialize_questions(&category.name);
                self.current_question_index = 0;
                self.score = 0;
                self.last_guess_time = None; // Reset guess timer
            }
        }
    }

    fn show_quiz(&mut self, ui: &mut egui::Ui, category: &Category) {
        // Score Counter at the top right
        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() - 100.0); // Align to the right
            ui.label(format!("Score: {}", self.score));
        });

        // Quiz content
        ui.separator(); // Separator between score and quiz content
        ui.heading(format!("{} Quiz", category.name));

        if self.current_question_index < self.current_questions.len() {
            let question = &self.current_questions[self.current_question_index];

            ui.vertical_centered(|ui| {
                // Display the question in the center with a custom font size
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(&question.question_text)
                        .size(24.0), // Font size for the question
                );
                ui.add_space(20.0);

                let mut guessed = false;

                // Display answer buttons with custom font size and color feedback
                for (i, option) in question.options.iter().enumerate() {
                    let is_correct = question.is_correct(i);

                    // Determine button color based on the guess state
                    let button_color = if let Some(guess_time) = self.last_guess_time {
                        if guess_time.elapsed() < Duration::from_secs(1) {
                            // Highlight the correct answer for 1 second
                            if is_correct {
                                egui::Color32::from_rgb(100, 255, 100) // Green for correct
                            } else {
                                egui::Color32::from_rgb(40, 40, 40) // Default color
                            }
                        } else {
                            guessed = true; // Move to the next question after 1 second
                            egui::Color32::from_rgb(40, 40, 40) // Reset to default
                        }
                    } else {
                        egui::Color32::from_rgb(40, 40, 40) // Default color
                    };

                    let button = egui::Button::new(
                        egui::RichText::new(option).size(20.0), // Font size for the answers
                    )
                    .fill(button_color) // Apply the determined button color
                    .wrap() // Enable text wrapping for long answers
                    .min_size(egui::Vec2::new(ui.available_width(), 40.0)); // Full width and height

                    if ui.add(button).clicked() && self.last_guess_time.is_none() {
                        // Handle the guess and set the timer
                        self.last_guess_time = Some(Instant::now());
                        if is_correct {
                            self.score += 1;
                        }
                    }
                    ui.add_space(10.0); // Add spacing between buttons
                }

                // Advance to the next question after showing feedback
                if guessed {
                    self.current_question_index += 1;
                    self.last_guess_time = None; // Reset the timer
                }
            });
        } else {
            // Quiz completed screen
            ui.label(
                egui::RichText::new(format!("Quiz completed! Your score: {}", self.score))
                    .size(20.0), // Font size for the completion message
            );
            if ui.button(
                egui::RichText::new("Back to Main Menu").size(18.0), // Font size for the button
            )
            .clicked()
            {
                // Reset quiz state
                self.selected_category = None;
                self.current_questions.clear();
                self.current_question_index = 0;
                self.score = 0;
                self.last_guess_time = None;
            }
        }
    }
}
