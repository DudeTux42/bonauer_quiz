#[derive(Debug)]
pub struct Question {
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

impl Question {
    pub fn new(question_text: String, options: Vec<String>, correct_answer: usize) -> Self {
        Question {
            question_text,
            options,
            correct_answer,
        }
    }

    pub fn is_correct(&self, answer: usize) -> bool {
        self.correct_answer == answer
    }
}

