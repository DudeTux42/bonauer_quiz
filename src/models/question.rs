use rand::prelude::SliceRandom;
use std::io::{self, Write};

#[derive(Clone, Debug)]
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

    pub fn ask(&self) -> bool {
        println!("\n{}", self.question_text);

        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        print!("Your answer (1-{}): ", self.options.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(answer) if answer > 0 && answer <= self.options.len() => {
                let is_correct = answer - 1 == self.correct_answer;
                if is_correct {
                    println!("Correct!");
                } else {
                    println!(
                        "Wrong! The correct answer was: {}",
                        self.options[self.correct_answer]
                    );
                }
                is_correct
            }
            _ => {
                println!(
                    "Invalid input. The correct answer was: {}",
                    self.options[self.correct_answer]
                );
                false
            }
        }
    }

    pub fn is_correct(&self, index: usize) -> bool {
        index == self.correct_answer
    }

    // Shuffle options and update the correct_answer index based on the new order
    pub fn shuffle_options(&mut self) {
        let correct_option = self.options[self.correct_answer].clone();
        // println!("Before shuffle: Correct option = {}", correct_option); TEST:
        // Shuffle the options
        let mut rng = rand::thread_rng();
        self.options.shuffle(&mut rng);

        // Find the new index of the correct answer
        self.correct_answer = self
            .options
            .iter()
            .position(|x| x == &correct_option)
            .unwrap();
        // println!("After shuffle: Correct option = {}", self.options[self.correct_answer]); TEST:
    }
}
