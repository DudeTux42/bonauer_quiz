use ipv4::convert_snm;

use crate::data::create_sample_quiz;
use crate::utils::{choose_category, read_input};

mod quiz;
mod category;
mod question;
mod data;
mod utils;
mod ipv4;

fn main() {
   loop{ 
        // let ipaddr = create_ipv4();
        // println!("Random IPV4: {:?}", ipaddr);
        // println!("Range: {}", range_to_str(ipv4_range(ipaddr)));
        // create quiz
        let cidr_input = read_input("Enter a value 1..32 as CIDR SNM: ");
        let cidr: u8 = match cidr_input.trim().parse() {
            Ok(num) if num >= 1 && num <= 32 => num,
            _ => {
                println!("Invalid input. Please enter a number between 1 and 32.");
                continue; // Startet die Schleife neu, wenn die Eingabe ungültig ist
            }
        };
        let snm = convert_snm(cidr);
        println!("The win SNM of /{} is {}", cidr, snm );
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

