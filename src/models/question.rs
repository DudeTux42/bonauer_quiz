// Import the necessary traits and modules
use crate::utils::random;
use rand::prelude::SliceRandom; // For shuffling the options
use std::io::{self, Write}; // For handling input and output operations // Import the random module for shuffling

#[derive(Clone)] // Allows the Question struct to be cloned
pub struct Question {
    pub question_text: String, // The question text itself
    pub options: Vec<String>,  // List of options for answers
    pub correct_answer: usize, // The index of the correct answer in `options`
}

impl Question {
    /// Creates a new Question with the given text, options, and correct answer index.
    pub fn new(question_text: String, options: Vec<String>, correct_answer: usize) -> Self {
        Question {
            question_text,  // Assign the given question text
            options,        // Assign the list of options
            correct_answer, // Assign the index of the correct answer
        }
    }

    /// Asks the question, displays the options, and checks if the user's answer is correct.
    /// Returns `true` if the answer is correct, otherwise `false`.
    pub fn ask(&self) -> bool {
        // Print the question text
        println!("\n{}", self.question_text);

        // Print each option with its index
        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        // Prompt the user for input and flush the output buffer
        print!("Your answer (1-{}): ", self.options.len());
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

        // Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the answer input from the user

        // Parse the input and check if it is valid (within the valid range)
        match input.trim().parse::<usize>() {
            // If input is valid and within the range of options
            Ok(answer) if answer > 0 && answer <= self.options.len() => {
                // Check if the selected answer is correct
                let is_correct = answer - 1 == self.correct_answer;
                if is_correct {
                    println!("Correct!"); // Inform the user if they were correct
                } else {
                    println!(
                        "Wrong! The correct answer was: {}",
                        self.options[self.correct_answer] // Display the correct answer
                    );
                }
                is_correct // Return whether the answer was correct
            }
            // If the input is invalid (outside the range or not a number)
            _ => {
                println!(
                    "Invalid input. The correct answer was: {}",
                    self.options[self.correct_answer] // Display the correct answer on invalid input
                );
                false // Return false since the answer was incorrect
            }
        }
    }

    /// Checks if a given answer index matches the correct answer.
    pub fn is_correct(&self, index: usize) -> bool {
        index == self.correct_answer // Return whether the index is the same as the correct answer index
    }

    /// Shuffles the options and adjusts the `correct_answer` index accordingly.
    pub fn shuffle_options(&mut self) {
        let mut rng = rand::thread_rng(); // Create a random number generator
        let correct_option = self.options[self.correct_answer].clone(); // Store the correct option

        // Shuffle the options randomly
        self.options.shuffle(&mut rng);

        // Find the new index of the correct option after shuffling
        self.correct_answer = self
            .options
            .iter()
            .position(|option| *option == correct_option) // Find the new position of the correct option
            .unwrap(); // This should always succeed since the correct option is in the list
    }
}
