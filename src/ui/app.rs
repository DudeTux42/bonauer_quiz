use crate::models::{Category, Question, Quiz};
use crate::ui::components;
use eframe::egui;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

/// Main application structure.
pub struct MyApp {
    quiz: Quiz,
    categories: Vec<Category>,
    selected_category: Option<Category>,
    current_questions: Vec<Question>,
    current_question_index: usize,
    score: usize,
    last_guess_time: Option<Instant>,
}

impl Default for MyApp {
    fn default() -> Self {
        let quiz = crate::data::create_sample_quiz();
        Self {
            categories: quiz.categories.clone().into_values().collect(),
            selected_category: None,
            current_questions: Vec::new(),
            current_question_index: 0,
            score: 0,
            quiz,
            last_guess_time: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Clone the selected_category to avoid holding an immutable borrow on self.
            let selected_category = self.selected_category.clone();
            match selected_category {
                None => self.draw_main_menu(ui),
                Some(category) => self.draw_quiz(ui, &category),
            }
        });
    }
}

impl MyApp {
    /// Draws the main menu where the user selects a category.
    fn draw_main_menu(&mut self, ui: &mut egui::Ui) {
        components::centered_text(ui, "Bonauer Quiz", 30.0);
        ui.label("Select a game mode:");

        ui.vertical_centered(|ui| {
            for category in &self.categories.clone() {
                // Use the reusable QuizButton component.
                let response = components::QuizButton::new(&category.name)
                    .size(20.0)
                    .show(ui);
                if response.clicked() {
                    self.selected_category = Some(category.clone());
                    println!("Selected category: {}", category.name);
                    self.setup_category(&category);
                }
                ui.add_space(10.0);
            }
        });
    }

    /// Prepares the selected category by selecting and shuffling questions.
    /// Special categories (e.g., "IPV4" and "Subnetting") are not shuffled.
    fn setup_category(&mut self, category: &Category) {
        if let Some(cat) = self.quiz.categories.get(&category.name) {
            let mut questions = cat.questions.clone();

            // For categories other than "IPV4" and "Subnetting", shuffle the questions.
            if category.name != "IPV4" && category.name != "Subnetting" {
                let mut rng = thread_rng();
                questions.shuffle(&mut rng);
                questions.truncate(10);
                // Shuffle the answer options for each question.
                for question in &mut questions {
                    question.shuffle_options();
                }
            } else {
                // For special categories, only truncate the questions.
                questions.truncate(10);
            }
            self.current_questions = questions;
        }
        self.current_question_index = 0;
        self.score = 0;
        self.last_guess_time = None;
    }

    /// Draws the quiz screen, including the question, answer buttons, score display, and navigation.
    fn draw_quiz(&mut self, ui: &mut egui::Ui, category: &Category) {
        // Display score using the reusable component.
        components::score_display(ui, self.score);
        // Display the quiz header with the category title.
        components::quiz_header(ui, &format!("{} Quiz", category.name), None, 20.0);
        ui.separator();

        if self.current_question_index < self.current_questions.len() {
            let question = &self.current_questions[self.current_question_index];
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                // Display the question text.
                ui.label(egui::RichText::new(&question.question_text).size(24.0));
                ui.add_space(20.0);

                let mut guessed = false;

                // Display each answer option using the AnswerOptionButton component.
                for (i, option) in question.options.iter().enumerate() {
                    let state = if let Some(guess_time) = self.last_guess_time {
                        if guess_time.elapsed() < Duration::from_millis(500) {
                            if question.is_correct(i) {
                                "correct"
                            } else {
                                "default"
                            }
                        } else {
                            guessed = true;
                            "default"
                        }
                    } else {
                        "default"
                    };

                    let response = components::AnswerOptionButton::new(option)
                        .size(20.0)
                        .show(ui, state);

                    if response.clicked() && self.last_guess_time.is_none() {
                        self.last_guess_time = Some(Instant::now());
                        if question.is_correct(i) {
                            self.score += 1;
                        }
                    }
                    ui.add_space(10.0);
                }

                if guessed {
                    self.current_question_index += 1;
                    self.last_guess_time = None;
                }
            });
            ui.ctx().request_repaint();
        } else {
            // Quiz finished.
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("Quiz completed! Your score: {}", self.score))
                        .size(30.0)
                        .strong(),
                );
                ui.add_space(30.0);

                // Button to restart the quiz for the current category.
                if ui
                    .button(egui::RichText::new("Restart Quiz").size(20.0))
                    .clicked()
                {
                    self.setup_category(category);
                }
                ui.add_space(10.0);

                // Button to return to the main menu.
                if ui
                    .button(egui::RichText::new("Back to Main Menu").size(18.0))
                    .clicked()
                {
                    self.selected_category = None;
                    self.current_questions.clear();
                    self.current_question_index = 0;
                    self.score = 0;
                    self.last_guess_time = None;
                }
            });
        }
    }
}
