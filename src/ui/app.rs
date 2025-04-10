use crate::data::quiz_data;
use crate::models::quiz::Quiz;
use crate::utils::time_utils::{self, duration_from_millis, now};
// Use the Instant and Duration from the appropriate source
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

use egui;
pub struct MyApp {
    quiz: Option<Quiz>,
    current_question: usize,
    selected_answer: Option<usize>,
    show_feedback: bool,
    score: usize,
    state: AppState,
    start_time: Option<Instant>,
    elapsed_time: Option<Duration>,
    next_question_requested: bool,
    selected_category: Option<String>,
    category_questions: Vec<crate::models::Question>,
    request_category_selection: Option<String>,
}

#[derive(PartialEq)]
enum AppState {
    MainMenu,
    CategorySelection,
    Playing,
    Results,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            quiz: None,
            current_question: 0,
            selected_answer: None,
            show_feedback: false,
            score: 0,
            state: AppState::MainMenu,
            start_time: None,
            elapsed_time: None,
            next_question_requested: false,
            selected_category: None,
            category_questions: Vec::new(),
            request_category_selection: None,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup custom fonts and styles if needed
        setup_custom_styles(&cc.egui_ctx);

        Self::default()
    }

    fn show_category_selection(&mut self) {
        // Load quiz with all categories
        self.quiz = Some(quiz_data::create_sample_quiz());
        self.state = AppState::CategorySelection;
    }

    fn start_quiz_for_category(&mut self, category_name: &str) {
        if let Some(quiz) = &self.quiz {
            // Get the category and its questions
            if let Some(category) = quiz.categories.get(category_name) {
                // Use the category's get_quiz_questions method to get shuffled questions
                let quiz_questions = category.get_quiz_questions();

                if !quiz_questions.is_empty() {
                    // Store the questions for this category
                    self.category_questions = quiz_questions;
                    self.current_question = 0;
                    self.selected_answer = None;
                    self.show_feedback = false;
                    self.score = 0;
                    self.state = AppState::Playing;
                    self.start_time = Some(now());
                    self.elapsed_time = None;
                    self.next_question_requested = false;
                    self.selected_category = Some(category_name.to_string());
                }
            }
        }
    }

    fn next_question(&mut self) {
        if self.current_question < self.category_questions.len() - 1 {
            self.current_question += 1;
            self.selected_answer = None;
            self.show_feedback = false;
        } else {
            self.state = AppState::Results;
            if let Some(start_time) = self.start_time {
                self.elapsed_time = Some(start_time.elapsed());
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Bonauer Quiz");

                match self.state {
                    AppState::MainMenu => self.render_main_menu(ui),
                    AppState::CategorySelection => self.render_category_selection(ui),
                    AppState::Playing => self.render_quiz(ui),
                    AppState::Results => self.render_results(ui),
                }
            });
        });

        // Handle next question request outside of UI closure
        if self.next_question_requested {
            self.next_question();
            self.next_question_requested = false;
        }

        // Handle category selection outside of UI closure
        if let Some(category) = self.request_category_selection.take() {
            self.start_quiz_for_category(&category);
        }
    }
}

impl MyApp {
    fn render_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            ui.label("Test your knowledge in various IT fields");

            ui.add_space(20.0);

            if ui.button("Start Quiz").clicked() {
                self.show_category_selection();
            }

            // Additional menu options could go here
        });
    }

    fn render_category_selection(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            ui.heading("Select a Category");
            ui.add_space(15.0);

            if let Some(quiz) = &self.quiz {
                // Get available categories from the quiz
                let categories = quiz.categories();

                if categories.is_empty() {
                    ui.label("No categories found. Please add some categories to the quiz.");
                } else {
                    // Create a button for each category
                    for category in categories {
                        if ui.button(category).clicked() {
                            // Store selection for processing outside the UI closure
                            self.request_category_selection = Some(category.to_string());
                        }
                    }
                }
            }

            ui.add_space(15.0);

            if ui.button("Back to Main Menu").clicked() {
                self.state = AppState::MainMenu;
            }
        });
    }

    fn render_quiz(&mut self, ui: &mut egui::Ui) {
        if self.current_question < self.category_questions.len() {
            let question = &self.category_questions[self.current_question];

            ui.add_space(10.0);

            // Display category if available
            if let Some(category) = &self.selected_category {
                ui.label(format!("Category: {}", category));
                ui.add_space(5.0);
            }

            // Question number and text
            ui.strong(format!(
                "Question {} of {}",
                self.current_question + 1,
                self.category_questions.len()
            ));
            ui.add_space(5.0);
            ui.label(&question.question_text);

            ui.add_space(15.0);

            // Answer options
            for (idx, option) in question.options.iter().enumerate() {
                let mut button = egui::Button::new(option);

                // If feedback is shown, highlight correct and incorrect answers
                if self.show_feedback {
                    if idx == question.correct_answer {
                        button = button.fill(egui::Color32::from_rgb(100, 200, 100));
                    } else if Some(idx) == self.selected_answer {
                        button = button.fill(egui::Color32::from_rgb(200, 100, 100));
                    }
                } else if Some(idx) == self.selected_answer {
                    button = button.fill(egui::Color32::from_rgb(150, 150, 200));
                }

                if ui.add(button).clicked() && !self.show_feedback {
                    self.selected_answer = Some(idx);
                }
            }

            ui.add_space(15.0);

            // Controls (Submit/Next)
            ui.horizontal(|ui| {
                if !self.show_feedback {
                    let submit_enabled = self.selected_answer.is_some();
                    if ui
                        .add_enabled(submit_enabled, egui::Button::new("Submit"))
                        .clicked()
                    {
                        self.show_feedback = true;

                        // Check if the answer is correct
                        if let Some(selected) = self.selected_answer {
                            if selected == question.correct_answer {
                                self.score += 1;
                            }
                        }
                    }
                } else {
                    // Show explanation if available
                    if let Some(explanation) = &question.explanation {
                        ui.vertical(|ui| {
                            ui.label("Explanation:");
                            ui.label(explanation);
                        });
                    }

                    // Use flag instead of directly calling next_question
                    if ui.button("Next").clicked() {
                        self.next_question_requested = true;
                    }
                }
            });
        }
    }

    fn render_results(&mut self, ui: &mut egui::Ui) {
        let total_questions = self.category_questions.len();

        ui.vertical_centered(|ui| {
            ui.heading("Quiz Results");

            ui.add_space(15.0);

            // Display category if available
            if let Some(category) = &self.selected_category {
                ui.label(format!("Category: {}", category));
                ui.add_space(10.0);
            }

            let score_text = format!("Score: {} out of {}", self.score, total_questions);
            ui.strong(score_text);

            if let Some(elapsed) = self.elapsed_time {
                ui.label(format!("Time: {:.1} seconds", elapsed.as_secs_f32()));
            }

            let percentage = if total_questions > 0 {
                (self.score as f32 / total_questions as f32) * 100.0
            } else {
                0.0
            };
            ui.label(format!("Percentage: {:.1}%", percentage));

            ui.add_space(10.0);

            // Display performance message based on percentage
            let message = if percentage >= 90.0 {
                "Excellent! You're a master!"
            } else if percentage >= 70.0 {
                "Great job! Well done!"
            } else if percentage >= 50.0 {
                "Good effort! Keep practicing!"
            } else {
                "Keep studying and try again!"
            };

            ui.label(message);

            ui.add_space(20.0);

            // Button to choose another category
            if ui.button("Choose Another Category").clicked() {
                self.show_category_selection();
            }

            if ui.button("Main Menu").clicked() {
                self.state = AppState::MainMenu;
            }
        });
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Customize font sizes
    style
        .text_styles
        .get_mut(&egui::TextStyle::Body)
        .unwrap()
        .size = 16.0;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Button)
        .unwrap()
        .size = 16.0;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Heading)
        .unwrap()
        .size = 24.0;

    // Apply the style changes
    ctx.set_style(style);
}
