use crate::category::Category;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Quiz {
    pub categories: HashMap<String, Category>,
}

impl Quiz {
    pub fn new() -> Self {
        Quiz {
            categories: HashMap::new(),
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.categories.insert(category.name.clone(), category);
    }

    pub fn get_category(&self, name: &str) -> Option<&Category> {
        self.categories.get(name)
    }

    pub fn take_quiz(&self, category_name: &str) -> usize {
        if let Some(category) = self.get_category(category_name) {
            let mut score = 0;
            for question in &category.questions {
                println!("{}", question.question_text);
                for (index, option) in question.options.iter().enumerate() {
                    println!("{}: {}", index + 1, option);
                }

                let mut user_answer = String::new();
                std::io::stdin().read_line(&mut user_answer).unwrap();
                let user_answer: usize = user_answer.trim().parse().unwrap();

                if question.is_correct(user_answer - 1) {
                    score += 1;
                }
            }
            score
        } else {
            println!("Category not found!");
            0
        }
    }
}

