// Import the `Question` struct from the `models` module
use crate::models::Question;
// Import the `SliceRandom` trait to enable shuffling of vectors
use rand::seq::SliceRandom;

#[derive(Clone)] // Allows the Category struct to be cloned
pub struct Category {
    pub name: String,             // Name of the category (e.g., "Science", "History")
    pub questions: Vec<Question>, // List of questions belonging to this category
}

impl Category {
    /// Creates a new `Category` with a given name and an empty list of questions.
    pub fn new(name: String) -> Self {
        Category {
            name,                  // Assign the given name
            questions: Vec::new(), // Initialize with an empty question list
        }
    }

    /// Adds a question to the category's question list.
    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    /// Conducts a quiz using up to 10 randomly selected questions.
    /// Returns the number of correctly answered questions.
    pub fn take_quiz(&self) -> usize {
        let mut score = 0; // Initialize the score counter

        // Create a random number generator
        let mut rng = rand::thread_rng();

        // Clone the questions so that the original list remains unmodified
        let mut quiz_questions = self.questions.clone();

        // Shuffle the list of questions randomly
        quiz_questions.shuffle(&mut rng);

        // Limit the quiz to a maximum of 10 questions
        quiz_questions.truncate(10);

        // Iterate over the selected questions and ask each one
        for question in quiz_questions {
            if question.ask() {
                score += 1; // Increment score if the question is answered correctly
            }
        }

        // Return the final score
        score
    }
}
