use crate::question::Question;

#[derive(Debug)]
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
}

