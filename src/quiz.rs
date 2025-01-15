use crate::category::Category;
use crate::question::Question;
use rand::seq::SliceRandom;
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

    pub fn initialize_questions(&self, category_name: &str) -> Vec<Question> {
        if let Some(category) = self.get_category(category_name) {
            let mut rng = rand::thread_rng();
            let questions = &category.questions;
            let mut selected_questions: Vec<_> = questions
                .choose_multiple(&mut rng, 10.min(questions.len()))
                .cloned()
                .collect();

            // Shuffle options for each selected question
            for question in &mut selected_questions {
                question.shuffle_options();
            }

            selected_questions
        } else {
            vec![]
        }
    }

    pub fn take_quiz(&self, category_name: &str) {
        let selected_questions = self.initialize_questions(category_name);
        let mut score = 0;

        for question in selected_questions {
            println!("{}", question.question_text);

            for (index, option) in question.options.iter().enumerate() {
                println!("{}: {}", index + 1, option);
            }

            let user_answer: usize = loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= question.options.len() => break num,
                    _ => println!("Please enter a valid option (1-{})", question.options.len()),
                }
            };

            if question.is_correct(user_answer - 1) {
                println!("Correct Answer!");
                score += 1;
            }
        }

        println!("Your score: {}", score);
    }
}
