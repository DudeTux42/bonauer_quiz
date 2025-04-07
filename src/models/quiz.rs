use crate::utils::random;
use rand::SeedableRng; // Keep this if you still need it

#[derive(Clone, Debug)]
pub struct Quiz {
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug)]
pub struct Question {
    pub question_text: String,
    pub answers: Vec<Answer>,
    pub explanation: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub text: String,
    pub correct: bool,
}

impl Quiz {
    pub fn new(title: String, description: String, questions: Vec<Question>) -> Self {
        Quiz {
            title,
            description,
            questions,
        }
    }

    pub fn shuffle_answers(&mut self) {
        for question in &mut self.questions {
            // Use our platform-independent shuffle
            random::shuffle(&mut question.answers);
        }
    }

    pub fn total_questions(&self) -> usize {
        self.questions.len()
    }
}

impl Question {
    pub fn new(question_text: String, answers: Vec<Answer>, explanation: Option<String>) -> Self {
        Question {
            question_text,
            answers,
            explanation,
        }
    }

    pub fn get_correct_answer_index(&self) -> Option<usize> {
        self.answers.iter().position(|answer| answer.correct)
    }
}

impl Answer {
    pub fn new(text: String, correct: bool) -> Self {
        Answer { text, correct }
    }
}
