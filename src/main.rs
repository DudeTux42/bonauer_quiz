use crate::data::create_sample_quiz;
use crate::utils::choose_category;
use crate::ipv4::{create_ipv4, ipv4_range, range_to_str};

mod quiz;
mod category;
mod question;
mod data;
mod utils;
mod ipv4;

fn main() {
   loop{ 
        let ipaddr = create_ipv4();
        println!("Random IPV4: {:?}", ipaddr);
        println!("Range: {}", range_to_str(ipv4_range(ipaddr)));
        // create quiz
        let quiz = create_sample_quiz();

        // category as list
        let categories = vec![
            "Mathematics".to_string(),
            "IT".to_string(),
            "Abbreviations".to_string(),
            "IPV4".to_string(),
        ];

        // user chooses a category
        let selected_category = choose_category(categories);

        // start quiz
        println!("You selected the {} category.", selected_category);

        let score = quiz.take_quiz(&selected_category);
        println!("Your score in the {} category: {}", selected_category, score);
    }
}

