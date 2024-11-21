use rand::seq::SliceRandom;

#[derive(Clone,Debug)]
pub struct Question {
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer: usize, // This will hold the index of the correct option in the shuffled list
}

impl Question {
    pub fn new(question_text: String, options: Vec<String>, correct_answer: usize) -> Self {
        Question {
            question_text,
            options,
            correct_answer,
        }
    }

    // Shuffle options and update the correct_answer index based on the new order
    pub fn shuffle_options(&mut self) {
        // Shuffle the options
        let mut rng = rand::thread_rng();
        self.options.shuffle(&mut rng);

        // Find the new index of the correct answer
        self.correct_answer = self.options.iter().position(|x| x == &self.options[self.correct_answer]).unwrap();
    }

    pub fn is_correct(&self, answer: usize) -> bool {
        self.correct_answer == answer
    }



}



