use eframe::egui;

mod models;
mod ui;
mod utils;
mod error;
mod data;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Bonauer Quiz GUI",
        options,
        Box::new(|_cc| Ok(Box::new(ui::MyApp::default()))),
    );
}























// use ipv4::{calculate_network_id, convert_snm, create_ipv4_and_snm, available_networks};
// use crate::data::create_sample_quiz;
// use crate::utils::{choose_category, read_input};
//
// mod quiz;
// mod category;
// mod question;
// mod data;
// mod utils;
// mod ipv4;
//
// fn main() {
//    loop{ 
//         // let ipaddr = create_ipv4();
//         // println!("Random IPV4: {:?}", ipaddr);
//         // println!("Range: {}", range_to_str(ipv4_range(ipaddr)));
//         // create quiz
//         let cidr_input = read_input("Enter a value 1..32 as CIDR SNM: ");
//         let cidr: u8 = match cidr_input.trim().parse() {
//             Ok(num) if num >= 1 && num <= 32 => num,
//             _ => {
//                 println!("Invalid input. Please enter a number between 1 and 32.");
//                 continue; // Startet die Schleife neu, wenn die Eingabe ungültig ist
//             }
//         };
//         let snm = convert_snm(cidr);
//         println!("The win SNM of /{} is {}", cidr, snm );
//         let (ip, prefix) = create_ipv4_and_snm();
//         println!("Genarated IP: {}, SNM: {}", ip, prefix);
//         let net_id = calculate_network_id(ip, prefix);
//         println!("Network ID: {}", net_id);
//         println!("{} available networks for {}/{}",available_networks(ip, prefix), ip, prefix);
//
//
//
//         let quiz = create_sample_quiz();
//
//         // category as list
//         let categories = vec![
//             "Mathematics".to_string(),
//             "IT".to_string(),
//             "Abbreviations".to_string(),
//             "IPV4".to_string(),
//         ];
//
//         // user chooses a category
//         let selected_category = choose_category(categories);
//
//         // start quiz
//         println!("You selected the {} category.", selected_category);
//
//         let score = quiz.take_quiz(&selected_category);
//         println!("Your score in the {} category: {}", selected_category, score);
//     }
// }
//
