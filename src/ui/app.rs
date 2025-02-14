// Import necessary modules and libraries
use crate::models::{Category, Question, Quiz}; // Import models for Category, Question, and Quiz
use crate::ui::components; // Import reusable UI components
use eframe::egui; // Import eframe and egui for GUI creation
use rand::seq::SliceRandom; // For shuffling the questions and options
use rand::thread_rng; // For creating a random number generator
use std::time::{Duration, Instant}; // For managing time-related functionality

/// Main application structure that holds the quiz logic and state.
pub struct MyApp {
    quiz: Quiz,                          // The Quiz data structure
    categories: Vec<Category>,           // List of available quiz categories
    selected_category: Option<Category>, // The currently selected category for the quiz
    current_questions: Vec<Question>,    // List of questions for the current quiz
    current_question_index: usize,       // Index for the current question in the quiz
    score: usize,                        // The current score of the user
    last_guess_time: Option<Instant>,    // The time of the last guess made by the user
}

impl Default for MyApp {
    /// Default method to initialize the application with a sample quiz.
    fn default() -> Self {
        let quiz = crate::data::create_sample_quiz(); // Generate a sample quiz
        Self {
            categories: quiz.categories.clone().into_values().collect(), // Extract categories from the quiz
            selected_category: None,       // No category selected initially
            current_questions: Vec::new(), // No questions selected initially
            current_question_index: 0,     // Start at the first question
            score: 0,                      // Initialize score to 0
            quiz,                          // Use the created quiz
            last_guess_time: None,         // No guesses made yet
        }
    }
}

impl eframe::App for MyApp {
    /// Update function to render the application UI on each frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Clone the selected category to avoid holding an immutable borrow on self.
            let selected_category = self.selected_category.clone();
            match selected_category {
                None => self.draw_main_menu(ui), // If no category selected, show main menu
                Some(category) => self.draw_quiz(ui, &category), // If category selected, show quiz
            }
        });
    }
}

impl MyApp {
    /// Draws the main menu where the user selects a quiz category.
    fn draw_main_menu(&mut self, ui: &mut egui::Ui) {
        components::centered_text(ui, "Bonauer Quiz", 30.0); // Display the title
        ui.label("Select a game mode:"); // Label for the category selection

        ui.vertical_centered(|ui| {
            for category in &self.categories.clone() {
                // Iterate through each available category
                // Use the reusable QuizButton component to display each category as a button.
                let response = components::QuizButton::new(&category.name)
                    .size(20.0) // Set the button size
                    .show(ui); // Show the button
                if response.clicked() {
                    // If the button is clicked
                    self.selected_category = Some(category.clone()); // Set the selected category
                    println!("Selected category: {}", category.name); // Log the selection
                    self.setup_category(&category); // Prepare the selected category for the quiz
                }
                ui.add_space(10.0); // Add space between the buttons
            }
        });
    }

    /// Prepares the selected category by shuffling or truncating questions as needed.
    fn setup_category(&mut self, category: &Category) {
        if let Some(cat) = self.quiz.categories.get(&category.name) {
            // Retrieve category data
            let mut questions = cat.questions.clone(); // Clone the questions for the selected category

            // For non-special categories, shuffle the questions and options
            if category.name != "IPV4" && category.name != "Subnetting" {
                let mut rng = thread_rng();
                questions.shuffle(&mut rng); // Shuffle the questions
                questions.truncate(10); // Limit to 10 questions
                for question in &mut questions {
                    question.shuffle_options(); // Shuffle the options for each question
                }
            } else {
                // For special categories, only truncate the questions without shuffling
                questions.truncate(10);
            }
            self.current_questions = questions; // Set the current questions to be asked
        }
        self.current_question_index = 0; // Reset the question index
        self.score = 0; // Reset the score
        self.last_guess_time = None; // Reset the guess time
    }

    /// Draws the quiz screen, including questions, answers, score, and navigation buttons.
    fn draw_quiz(&mut self, ui: &mut egui::Ui, category: &Category) {
        components::score_display(ui, self.score); // Display the current score
        components::quiz_header(ui, &format!("{} Quiz", category.name), None, 20.0); // Display the quiz header
        ui.separator(); // Add a separator

        if self.current_question_index < self.current_questions.len() {
            // If there are more questions
            let question = &self.current_questions[self.current_question_index]; // Get the current question
            ui.vertical_centered(|ui| {
                ui.add_space(10.0); // Add some space before the question
                ui.label(egui::RichText::new(&question.question_text).size(24.0)); // Display the question
                ui.add_space(20.0); // Add space before the options

                let mut guessed = false; // Flag to track if the user has guessed the answer

                // Display answer options
                for (i, option) in question.options.iter().enumerate() {
                    // Determine if the answer has been guessed and what state it should have
                    let state = if let Some(guess_time) = self.last_guess_time {
                        if guess_time.elapsed() < Duration::from_millis(500) {
                            if question.is_correct(i) {
                                "correct"
                            } else {
                                "default"
                            }
                        } else {
                            guessed = true;
                            "default"
                        }
                    } else {
                        "default"
                    };

                    // Use the AnswerOptionButton component to display each answer option
                    let response = components::AnswerOptionButton::new(option)
                        .size(20.0)
                        .show(ui, state);

                    // If the answer is clicked and it's the first guess, update the score and guess time
                    if response.clicked() && self.last_guess_time.is_none() {
                        self.last_guess_time = Some(Instant::now());
                        if question.is_correct(i) {
                            self.score += 1; // Increment score if the answer is correct
                        }
                    }
                    ui.add_space(10.0); // Add space between the options
                }

                if guessed {
                    // If the user has made a guess, move to the next question
                    self.current_question_index += 1;
                    self.last_guess_time = None;
                }
            });
            ui.ctx().request_repaint(); // Request a repaint to update the UI
        } else {
            // Quiz finished, show the result
            ui.vertical_centered(|ui| {
                ui.add_space(20.0); // Add space before the result message
                ui.label(
                    egui::RichText::new(format!("Quiz completed! Your score: {}", self.score))
                        .size(30.0)
                        .strong(), // Display the final score
                );
                ui.add_space(30.0); // Add space before buttons

                // Button to restart the quiz for the current category
                if ui
                    .button(egui::RichText::new("Restart Quiz").size(20.0))
                    .clicked()
                {
                    self.setup_category(category); // Reset and restart the quiz
                }
                ui.add_space(10.0); // Add space between buttons

                // Button to return to the main menu
                if ui
                    .button(egui::RichText::new("Back to Main Menu").size(18.0))
                    .clicked()
                {
                    self.selected_category = None; // Reset category selection
                    self.current_questions.clear(); // Clear current questions
                    self.current_question_index = 0; // Reset question index
                    self.score = 0; // Reset score
                    self.last_guess_time = None; // Reset guess time
                }
            });
        }
    }
}
