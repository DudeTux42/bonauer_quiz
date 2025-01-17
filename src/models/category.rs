use crate::question::Question;

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub questions: Vec<Question>,
    pub _subcategories: Vec<Category>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category {
            name,
            questions: Vec::new(),
            _subcategories: Vec::new(),

        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn _add_subcategory(&mut self, subcategory: Category) {
        self._subcategories.push(subcategory);
    }

    pub fn _get_subcategory(&self, name: &str) -> Option<&Category> {
        self._subcategories.iter().find(|sub| sub.name == name)
    }
}

