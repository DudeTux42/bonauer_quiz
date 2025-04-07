#[derive(Clone, Debug)]
pub struct Answer {
    pub text: String,
    pub correct: bool,
}

impl Answer {
    pub fn new(text: String, correct: bool) -> Self {
        Answer { text, correct }
    }
}
