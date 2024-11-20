use std::io::{self, Write};

// pub fn read_input(prompt: &str) -> String {
//     print!("{}", prompt);
//     io::stdout().flush().unwrap();
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     input.trim().to_string()
// }

pub fn choose_category(categories: Vec<String>) -> String {
    println!("Please choose a category by entering the corresponding number:");

    for (index, category) in categories.iter().enumerate() {
        println!("{}: {}", index + 1, category);
    }

    let mut choice = String::new();
    io::stdout().flush().unwrap(); // Damit der Benutzer aufgefordert wird
    io::stdin().read_line(&mut choice).unwrap();

    // Benutzereingabe in eine Zahl umwandeln
    let choice: usize = choice.trim().parse().unwrap_or(0);

    // Überprüfen, ob die Auswahl gültig ist
    if choice > 0 && choice <= categories.len() {
        categories[choice - 1].to_string()
    } else {
        println!("Invalid choice, please try again.");
        choose_category(categories) // Rekursive Wiederholung bei ungültiger Auswahl
    }
}
