use std::io::{self, Write};
use std::net::Ipv4Addr;

// function to read input from the user
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Get a number input from the user with a prompt
pub fn get_user_number(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    io::stdout().flush().unwrap_or_default();

    let input = read_input("");
    input.parse().ok()
}

// Clear the terminal screen
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap_or_default();
}

// Wait for user to press Enter
pub fn wait_for_key() {
    println!("\nPress Enter to continue...");
    read_input("");
}

// a function for choosing a category
pub fn choose_category(categories: Vec<String>) -> String {
    println!("Please choose a category by entering the corresponding number:");

    for (index, category) in categories.iter().enumerate() {
        println!("{}: {}", index + 1, category);
    }

    let mut choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();

    // convert user input to int
    let choice: usize = choice.trim().parse().unwrap_or(0);

    // check if selection is true
    if choice > 0 && choice <= categories.len() {
        categories[choice - 1].to_string()
    } else {
        println!("Invalid choice, please try again.");
        choose_category(categories) // repeate recursevly if selection is unvalid
    }
}

// a function that returns the first three and last three elements of a vector

pub fn first_and_last_three(vec: &[Ipv4Addr]) -> Vec<Ipv4Addr> {
    let mut result = Vec::new();
    if vec.len() >= 3 {
        result.extend_from_slice(&vec[..3]); // Die ersten drei
        result.extend_from_slice(&vec[vec.len() - 3..]); // Die letzten drei
    } else {
        result.extend_from_slice(vec);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last_three_subnets() {
        // Test case with more than 3 elements
        let vec = vec![
            Ipv4Addr::new(192, 168, 1, 0),
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv4Addr::new(192, 168, 1, 2),
            Ipv4Addr::new(192, 168, 1, 3),
            Ipv4Addr::new(192, 168, 1, 4),
            Ipv4Addr::new(192, 168, 1, 5),
            Ipv4Addr::new(192, 168, 1, 6),
            Ipv4Addr::new(192, 168, 1, 7),
            Ipv4Addr::new(192, 168, 1, 8),
        ];
        let result = first_and_last_three(&vec);
        assert_eq!(
            result,
            vec![
                Ipv4Addr::new(192, 168, 1, 0),
                Ipv4Addr::new(192, 168, 1, 1),
                Ipv4Addr::new(192, 168, 1, 2),
                Ipv4Addr::new(192, 168, 1, 6),
                Ipv4Addr::new(192, 168, 1, 7),
                Ipv4Addr::new(192, 168, 1, 8),
            ]
        );

        // Test case with exactly 3 elements
        let vec = vec![
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv4Addr::new(10, 0, 0, 2),
            Ipv4Addr::new(10, 0, 0, 3),
        ];
        let result = first_and_last_three(&vec);
        assert_eq!(
            result,
            vec![
                Ipv4Addr::new(10, 0, 0, 1),
                Ipv4Addr::new(10, 0, 0, 2),
                Ipv4Addr::new(10, 0, 0, 3),
            ]
        );

        // Test case with fewer than 3 elements
        let vec = vec![Ipv4Addr::new(192, 0, 2, 1), Ipv4Addr::new(192, 0, 2, 2)];
        let result = first_and_last_three(&vec);
        assert_eq!(
            result,
            vec![Ipv4Addr::new(192, 0, 2, 1), Ipv4Addr::new(192, 0, 2, 2),]
        );

        // Test case with an empty vector
        let vec: Vec<Ipv4Addr> = vec![];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![]);

        // Test case with a single element
        let vec = vec![Ipv4Addr::new(203, 0, 113, 5)];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![Ipv4Addr::new(203, 0, 113, 5)]);
    }

    #[test]
    fn test_read_input() {
        // Note: This test is more difficult to implement because it requires simulating user input
        // You might want to use a mock or test through integration tests instead
    }

    #[test]
    fn test_get_user_number() {
        // Similar to read_input, this would require simulating user input
        // Best tested through integration tests
    }

    #[test]
    fn test_choose_category() {
        // This would also require simulating user input
        // Best tested through integration tests
    }
}
