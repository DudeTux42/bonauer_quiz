use crate::models::Question;
use crate::utils::random;

#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category {
            name,
            questions: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    // Replace the take_quiz method with a method that uses our random module
    pub fn get_quiz_questions(&self) -> Vec<Question> {
        // Clone the questions so the original list remains unmodified
        let mut quiz_questions = self.questions.clone();

        // Use our WASM-compatible shuffle
        random::shuffle(&mut quiz_questions);

        // Limit the quiz to 10 questions
        quiz_questions.truncate(10);

        quiz_questions
    }
}
