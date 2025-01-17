use std::io::{self, Write};
// function to read input from the user
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
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
pub fn first_and_last_three(vec: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    result.extend_from_slice(&vec[0..std::cmp::min(3, vec.len())]);

    result.extend_from_slice(&vec[vec.len().saturating_sub(3)..]);

    result
}



#[cfg(test)]
mod tests {
    use super::*; // Import the function to be tested

    #[test]
    fn test_first_and_last_three() {
        // Test case with more than 3 elements
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![1, 2, 3, 7, 8, 9]);

        // Test case with exactly 3 elements
        let vec = vec![1, 2, 3];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![1, 2, 3, 1, 2, 3]);

        // Test case with fewer than 3 elements
        let vec = vec![1, 2];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![1, 2, 1, 2]);

        // Test case with an empty vector
        let vec: Vec<i32> = vec![];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![]);

        // Test case with a single element
        let vec = vec![42];
        let result = first_and_last_three(&vec);
        assert_eq!(result, vec![42, 42]);
    }
}
