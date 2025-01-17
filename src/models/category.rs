use crate::models::Question;
use rand::seq::SliceRandom;

#[derive(Clone)]
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

    pub fn take_quiz(&self) -> usize {
        let mut score = 0;
        let mut rng = rand::thread_rng();
        let mut quiz_questions = self.questions.clone();
        quiz_questions.shuffle(&mut rng);
        quiz_questions.truncate(10);

        for question in quiz_questions {
            if question.ask() {
                score += 1;
            }
        }
        score
    }
}
