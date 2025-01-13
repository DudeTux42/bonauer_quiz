use eframe::egui;
use std::collections::HashMap;
use crate::data::create_sample_quiz;
use crate::category::Category;

pub struct MyApp {
    categories: Vec<Category>,
    selected_category: Option<Category>,
}

impl Default for MyApp {
    fn default() -> Self {
        let quiz = create_sample_quiz();
        Self {
            categories: quiz.categories.into_values().collect(),
            selected_category: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bonauer Quiz");
            ui.label("Select a game mode:");

            for category in &self.categories {
                if ui.button(&category.name).clicked() {
                    self.selected_category = Some(category.clone());
                    println!("{} mode selected", category.name);
                }
            }

            if let Some(category) = &self.selected_category {
                ui.label(format!("Current mode: {}", category.name));
            }
        });
    }
}


//Test comment to test lazygit
