use crate::models::Question;
use rand::Rng;

pub fn generate_ipv4_question() -> Question {
    let mut rng = rand::thread_rng();
    
    // Generate a random IPv4 address
    let ip = generate_random_ip(&mut rng);
    
    // Calculate the correct class
    let class = determine_ip_class(&ip);
    
    // Create question text
    let question_text = format!("What is the class of the following IPv4 address: {}", ip);
    
    // Create answer options
    let options = vec![
        "Class A".to_string(),
        "Class B".to_string(),
        "Class C".to_string(),
        "Class D".to_string(),
        "Class E".to_string(),
    ];
    
    // Determine correct answer index
    let correct_answer = match class {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => 0, // Fallback, sollte nie passieren
    };
    
    Question::new(question_text, options, correct_answer)
}

fn generate_random_ip(rng: &mut impl Rng) -> String {
    let first_octet = match rng.gen_range(0..5) {
        0 => rng.gen_range(0..127),    // Class A
        1 => rng.gen_range(128..191),  // Class B
        2 => rng.gen_range(192..223),  // Class C
        3 => rng.gen_range(224..239),  // Class D
        4 => rng.gen_range(240..255),  // Class E
        _ => unreachable!(),
    };
    
    format!(
        "{}.{}.{}.{}",
        first_octet,
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256)
    )
}

fn determine_ip_class(ip: &str) -> char {
    if let Some(first_octet) = ip.split('.').next() {
        if let Ok(value) = first_octet.parse::<u8>() {
            match value {
                0..=127 => return 'A',
                128..=191 => return 'B',
                192..=223 => return 'C',
                224..=239 => return 'D',
                240..=255 => return 'E',
            }
        }
    }
    'A' // Fallback, sollte nie passieren
}

pub fn is_valid_ip(ip: &str) -> bool {
    let octets: Vec<&str> = ip.split('.').collect();
    
    if octets.len() != 4 {
        return false;
    }
    
    for octet in octets {
        if let Ok(value) = octet.parse::<u8>() {
            if value > 255 {
                return false;
            }
        } else {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_class_determination() {
        assert_eq!(determine_ip_class("126.0.0.1"), 'A');
        assert_eq!(determine_ip_class("128.0.0.1"), 'B');
        assert_eq!(determine_ip_class("192.168.0.1"), 'C');
        assert_eq!(determine_ip_class("224.0.0.1"), 'D');
        assert_eq!(determine_ip_class("240.0.0.1"), 'E');
    }

    #[test]
    fn test_valid_ip() {
        assert!(is_valid_ip("192.168.0.1"));
        assert!(is_valid_ip("255.255.255.255"));
        assert!(is_valid_ip("0.0.0.0"));
        
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
    }

    #[test]
    fn test_ipv4_question_generation() {
        let question = generate_ipv4_question();
        
        // Check if question contains "IPv4" and "class"
        assert!(question.question_text.contains("IPv4"));
        assert!(question.question_text.contains("class"));
        
        // Check if we have exactly 5 options (Class A through E)
        assert_eq!(question.options.len(), 5);
        
        // Check if options are correctly formatted
        assert!(question.options.iter().all(|opt| opt.starts_with("Class ")));
        
        // Check if correct_answer is within valid range
        assert!(question.correct_answer < 5);
    }
}
