use eframe::egui;
use crate::data::create_sample_quiz;
use crate::category::Category;
use crate::question::Question;
use crate::quiz::Quiz;

pub struct MyApp {
    categories: Vec<Category>,
    selected_category: Option<Category>,
    current_questions: Vec<Question>,
    current_question_index: usize,
    score: usize,
    quiz: Quiz,
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
            }
        }
    }

    fn show_quiz(&mut self, ui: &mut egui::Ui, category: &Category) {
        ui.heading(format!("{} Quiz", category.name));

        if self.current_question_index < self.current_questions.len() {
            let question = &self.current_questions[self.current_question_index];

            ui.label(&question.question_text);

            for (i, option) in question.options.iter().enumerate() {
                if ui.button(option).clicked() {
                    if question.is_correct(i) {
                        self.score += 1;
                    }
                    self.current_question_index += 1;
                }
            }
        } else {
            ui.label(format!("Quiz completed! Your score: {}", self.score));
            if ui.button("Back to Main Menu").clicked() {
                self.selected_category = None;
                self.current_questions.clear();
                self.current_question_index = 0;
                self.score = 0;
            }
        }
    }
}
