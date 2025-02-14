use crate::models::{Category, Question};
use rand::prelude::SliceRandom; // Import SliceRandom to shuffle the questions
use rand::thread_rng; // Import thread_rng to create a random number generator
use std::collections::HashMap; // Import HashMap to store categories // Import Category and Question from the models module

#[derive(Clone)] // Derive Clone to allow copying of Quiz instances
pub struct Quiz {
    pub categories: HashMap<String, Category>, // Store quiz categories using a HashMap
}

impl Quiz {
    // Constructor function to create a new Quiz instance
    pub fn new() -> Self {
        Quiz {
            categories: HashMap::new(), // Initialize an empty HashMap for categories
        }
    }

    // Method to add a new category to the quiz
    pub fn add_category(&mut self, category: Category) {
        self.categories.insert(category.name.clone(), category); // Insert category into HashMap
    }

    // Method to initialize and shuffle questions from a selected category
    pub fn initialize_questions(&self, category_name: &str) -> Vec<Question> {
        if let Some(category) = self.categories.get(category_name) {
            let mut questions = category.questions.clone(); // Clone questions from the category
            let mut rng = rand::thread_rng(); // Create a random number generator
            questions.shuffle(&mut rng); // Shuffle the questions randomly
            questions.truncate(10); // Limit the question list to 10 questions
            questions // Return the shuffled list of 10 questions
        } else {
            Vec::new() // Return an empty vector if the category does not exist
        }
    }

    // Method to start the quiz for a given category and return the score
    pub fn take_quiz(&self, category_name: &str) -> usize {
        if let Some(category) = self.categories.get(category_name) {
            category.take_quiz() // Call the take_quiz method from the Category struct
        } else {
            0 // Return 0 if the category does not exist
        }
    }
}
