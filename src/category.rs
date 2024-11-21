use crate::question::Question;

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub questions: Vec<Question>,
    pub subcategories: Vec<Category>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category {
            name,
            questions: Vec::new(),
            subcategories: Vec::new(),

        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn add_subcategory(&mut self, subcategory: Category) {
        self.subcategories.push(subcategory);
    }

    pub fn get_subcategory(&self, name: &str) -> Option<&Category> {
        self.subcategories.iter().find(|sub| sub.name == name)
    }
}

