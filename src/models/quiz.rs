use std::collections::HashMap;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::models::{Category, Question};

#[derive(Clone)]
pub struct Quiz {
    pub categories: HashMap<String, Category>,
}

impl Quiz {
    pub fn new() -> Self {
        Quiz {
            categories: HashMap::new(),
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.categories.insert(category.name.clone(), category);
    }

    pub fn initialize_questions(&self, category_name: &str) -> Vec<Question> {
        if let Some(category) = self.categories.get(category_name) {
            let mut questions = category.questions.clone();
            let mut rng = rand::thread_rng();
            questions.shuffle(&mut rng);
            questions.truncate(10); // Limitiere auf 10 Fragen
            questions
        } else {
            Vec::new()
        }
    }

    pub fn take_quiz(&self, category_name: &str) -> usize {
        if let Some(category) = self.categories.get(category_name) {
            let mut questions = category.questions.clone();
            let mut rng = thread_rng();
            questions.shuffle(&mut rng);

            let mut score = 0;
            for question in questions {
                if question.ask() {
                    score += 1;
                }
            }
            score
        } else {
            0
        }
    }
}
