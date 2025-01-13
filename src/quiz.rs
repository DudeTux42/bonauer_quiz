use crate::category::Category;
use crate::ipv4::generate_subnet_snm_question;
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

    // a function to create a quiz
    pub fn take_quiz(&self, category_name: &str) -> usize {
    if let Some(category) = self.get_category(category_name) {
        let mut score = 0;

        // Zufällige Auswahl von 10 Fragen
        let mut rng = rand::thread_rng();
        let questions = &category.questions;
        let selected_questions: Vec<_> = questions
            .choose_multiple(&mut rng, 10.min(questions.len())) // Wähle 10 oder weniger Fragen
            .collect();
        // for loop to iterate over the selected questions and display them
        for question in selected_questions {
            println!("{}", question.question_text);
            let mut q = question.clone();
            q.shuffle_options();

            for (index, option) in q.options.iter().enumerate() {
                println!("{}: {}", index + 1, option);
            }

            let user_answer: usize = loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= q.options.len() => break num,
                    _ => println!("Please enter a valid option (1-{})", q.options.len()),
                }
            };

            if q.is_correct(user_answer - 1) {
                    println!("Correct Answer!");
                score += 1;
            }
        }
        score
    } else {
        println!("Category not found!");
        0
    }
}






    pub fn take_quiz_ordered(&self, category_name: &str) -> usize {
        if let Some(category) = self.get_category(category_name) {
            let mut score = 0;
            for question in &category.questions {
                println!("{}", question.question_text);
                let mut q = question.clone();
                q.shuffle_options();

                for (index, option) in q.options.iter().enumerate() {
                    println!("{}: {}", index + 1, option);
                }

                let user_answer: usize = loop {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse::<usize>() {
                        Ok(num) if num > 0 && num <= q.options.len() => break num,
                        _ => println!("Please enter a valid option (1-{})", q.options.len()),
                    }
                };

                if q.is_correct(user_answer - 1) {
                    score += 1;
                }
            }
            score
        } else {
            println!("Category not found!");
            0
        }
    }

    pub fn take_quiz_with_subnet_question() {
        let question = generate_subnet_snm_question();
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
        } else {
            println!("Incorrect Answer!");
        }
    }


}





