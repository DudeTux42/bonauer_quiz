use crate::models::Category;
use crate::utils::random;
use std::collections::HashMap;

#[derive(Clone, Debug)]
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
            // Use the category's method to get quiz questions
            category.get_quiz_questions()
        } else {
            Vec::new()
        }
    }

    pub fn shuffle_answers(&mut self) {
        for (_, category) in &mut self.categories {
            for question in &mut category.questions {
                // Adapt this to match your Question structure
                // If your Question struct has an 'options' vector, use:
                random::shuffle(&mut question.options);
            }
        }
    }
}
