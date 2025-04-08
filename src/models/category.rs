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

    pub fn get_quiz_questions(&self) -> Vec<Question> {
        let mut quiz_questions = self.questions.clone();
        random::shuffle(&mut quiz_questions);
        quiz_questions.truncate(10);
        quiz_questions
    }
}
