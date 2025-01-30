use crate::models::{Category, Question, Quiz};
use eframe::egui;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

pub struct MyApp {
    categories: Vec<Category>,
    selected_category: Option<Category>,
    current_questions: Vec<Question>,
    current_question_index: usize,
    score: usize,
    quiz: Quiz,
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

        let category_names: Vec<String> = self
            .categories
            .iter()
            .map(|category| category.name.clone())
            .collect();

        ui.vertical_centered(|ui| {
            for (i, category_name) in category_names.iter().enumerate() {
                let button = egui::Button::new(egui::RichText::new(category_name).size(20.0))
                    .fill(egui::Color32::from_rgb(40, 40, 40))
                    .min_size(egui::Vec2::new(ui.available_width(), 40.0));

                if ui.add(button).clicked() {
                    self.selected_category = Some(self.categories[i].clone());
                    println!("Selected category: {}", category_name); // Print the selected category
                    self.select_and_shuffle_questions(category_name);
                }
                ui.add_space(10.0);
            }
        });
    }

    fn select_and_shuffle_questions(&mut self, category_name: &str) {
        if let Some(cat) = self.quiz.categories.get(category_name) {
            let mut questions = cat.questions.clone();

            if category_name != "IPV4" {
                let mut rng = thread_rng();
                questions.shuffle(&mut rng);
                questions.truncate(10);

                // Shuffle the options for each truncated question
                for question in &mut questions {
                    question.shuffle_options();
                }
            } else {
                questions.truncate(10); // Only truncate without shuffling for ipv4_category
            }

            self.current_questions = questions;
        }
        self.current_question_index = 0;
        self.score = 0;
    }

    fn show_quiz(&mut self, ui: &mut egui::Ui, category: &Category) {
        // Score Counter at the top right
        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() - 100.0);
            ui.label(format!("Score: {}", self.score));
        });

        ui.separator();
        ui.heading(format!("{} Quiz", category.name));

        if self.current_question_index < self.current_questions.len() {
            let question = &self.current_questions[self.current_question_index];

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&question.question_text).size(24.0));
                ui.add_space(20.0);

                let mut guessed = false;

                for (i, option) in question.options.iter().enumerate() {
                    let is_correct = question.is_correct(i);

                    let button_color = if let Some(guess_time) = self.last_guess_time {
                        if guess_time.elapsed() < Duration::from_millis(500) {
                            if is_correct {
                                egui::Color32::from_rgb(100, 255, 100)
                            } else {
                                egui::Color32::from_rgb(40, 40, 40)
                            }
                        } else {
                            guessed = true;
                            egui::Color32::from_rgb(40, 40, 40)
                        }
                    } else {
                        egui::Color32::from_rgb(40, 40, 40)
                    };

                    let button = egui::Button::new(egui::RichText::new(option).size(20.0))
                        .fill(button_color)
                        .wrap()
                        .min_size(egui::Vec2::new(ui.available_width(), 40.0));

                    if ui.add(button).clicked() && self.last_guess_time.is_none() {
                        self.last_guess_time = Some(Instant::now());
                        if is_correct {
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
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("Quiz completed! Your score: {}", self.score))
                        .size(30.0)
                        .strong(),
                );

                ui.add_space(30.0);

                if ui
                    .button(egui::RichText::new("Restart Quiz").size(20.0))
                    .clicked()
                {
                    if let Some(cat) = self.quiz.categories.get(&category.name) {
                        let mut questions = cat.questions.clone();
                        let mut rng = thread_rng();
                        questions.shuffle(&mut rng);
                        questions.truncate(10);
                        self.current_questions = questions;
                    }
                    self.current_question_index = 0;
                    self.score = 0;
                    self.last_guess_time = None;
                }

                ui.add_space(10.0);

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
