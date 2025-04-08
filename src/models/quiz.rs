use crate::models::Category;
use crate::models::Question;
use crate::utils::random;
use std::collections::HashMap;

/// Quiz model that contains multiple categories of questions
#[derive(Clone, Debug)]
pub struct Quiz {
    /// Categories map, keyed by name
    pub categories: HashMap<String, Category>,
    /// Direct access to questions for compatibility with old code
    pub questions: Vec<Question>,
}

impl Quiz {
    /// Creates a new empty quiz
    pub fn new() -> Self {
        Quiz {
            categories: HashMap::new(),
            questions: Vec::new(),
        }
    }

    /// Add a category to the quiz
    pub fn add_category(&mut self, category: Category) {
        // Also add the category's questions to our direct questions list
        self.questions.extend(category.questions.clone());
        self.categories.insert(category.name.clone(), category);
    }

    /// Get questions for a specific category name
    pub fn initialize_questions(&self, category_name: &str) -> Vec<Question> {
        if let Some(category) = self.categories.get(category_name) {
            // Use the category's method to get quiz questions
            category.get_quiz_questions()
        } else {
            Vec::new()
        }
    }

    /// Shuffles the answer options for all questions in all categories
    pub fn shuffle_answers(&mut self) {
        // Shuffle answers in the direct questions list
        for question in &mut self.questions {
            question.shuffle_options();
        }

        // Shuffle answers in each category
        for (_, category) in &mut self.categories {
            for question in &mut category.questions {
                question.shuffle_options();
            }
        }
    }

    /// Get a list of all category names in the quiz
    pub fn categories(&self) -> Vec<&str> {
        self.categories.keys().map(|k| k.as_str()).collect()
    }

    /// Get the total number of questions in the direct questions list
    pub fn total_questions(&self) -> usize {
        self.questions.len()
    }

    /// Randomly shuffle all questions in the quiz
    pub fn shuffle_questions(&mut self) {
        random::shuffle(&mut self.questions);
    }
}
