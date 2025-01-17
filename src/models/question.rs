use rand::seq::SliceRandom;

#[derive(Clone,Debug)]
pub struct Question {
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer: usize, // This will hold the index of the correct option in the shuffled list
    pub options_shuffled: bool,
}

impl Question {
    pub fn new(question_text: String, options: Vec<String>, correct_answer: usize) -> Self {
        Question {
            question_text,
            options,
            correct_answer,
            options_shuffled: false,
        }
    }

    // Shuffle options and update the correct_answer index based on the new order
    pub fn shuffle_options(&mut self) {
        let correct_option = self.options[self.correct_answer].clone();
        // println!("Before shuffle: Correct option = {}", correct_option); TEST:
        // Shuffle the options
        let mut rng = rand::thread_rng();
        self.options.shuffle(&mut rng);
        self.options_shuffled = true;

        // Find the new index of the correct answer
        self.correct_answer = self.options.iter().position(|x| x == &correct_option).unwrap();
        // println!("After shuffle: Correct option = {}", self.options[self.correct_answer]); TEST:
    }

    pub fn is_correct(&self, answer: usize) -> bool {
        self.correct_answer == answer
    }



}



