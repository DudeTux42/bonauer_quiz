use crate::data::create_sample_quiz;
use crate::utils::choose_category;

mod quiz;
mod category;
mod question;
mod data;
mod utils;

fn main() {
   loop{ 
        // create quiz
        let quiz = create_sample_quiz();

        // category as list
        let categories = vec![
            "Mathematics".to_string(),
            "IT".to_string(),
            "Abbreviations".to_string(),
        ];

        // user chooses a category
        let selected_category = choose_category(categories);

        // start quiz
        println!("You selected the {} category.", selected_category);

        let score = quiz.take_quiz(&selected_category);
        println!("Your score in the {} category: {}", selected_category, score);
    }
}

