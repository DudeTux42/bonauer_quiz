use crate::models::Question; // Use your existing Question type
use crate::utils::random;
use std::net::Ipv4Addr;

pub fn generate_subnetting_question() -> Question {
    // Generate a random prefix length between 24 and 30
    let prefix = random::gen_range(24..=30) as u8;

    // Generate a random private network IP address
    let network_ip = generate_random_network_ip(prefix, false);

    // Create question text
    let min_networks = 6;
    let mut max_networks = 2u32.pow(32 - prefix as u32);
    if max_networks < min_networks {
        max_networks = min_networks;
    }
    let wanted_networks = random::gen_range(min_networks as u64..=max_networks as u64) as u32;

    let question_text = format!(
        "Bilde {} neue Netze aus dem folgenden Netzwerk:\n {}/{}",
        wanted_networks, network_ip, prefix,
    );

    // Create answer options
    let options = vec![];

    // Create question using your existing format - adapting to your pattern
    // Update this based on your actual Question constructor
    Question::new(question_text, options, 0)
}

pub fn generate_ipv4_question() -> Question {
    // Generate a random IPv4 address
    let ip = generate_random_ip();

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
        _ => 0, // Fallback, should never happen
    };

    // Use your existing Question constructor pattern
    Question::new(question_text, options, correct_answer)
}

fn generate_random_ip() -> String {
    // Generate class randomly (0-4)
    let class_type = random::gen_range(0..=4) as u8;

    let first_octet = match class_type {
        0 => random::gen_range(1..=126) as u8,   // Class A
        1 => random::gen_range(128..=191) as u8, // Class B
        2 => random::gen_range(192..=223) as u8, // Class C
        3 => random::gen_range(224..=239) as u8, // Class D
        4 => random::gen_range(240..=255) as u8, // Class E
        _ => 10,                                 // Fallback, should never happen
    };

    format!(
        "{}.{}.{}.{}",
        first_octet,
        random::gen_range(0..=255) as u8,
        random::gen_range(0..=255) as u8,
        random::gen_range(0..=255) as u8
    )
}

fn generate_random_network_ip(prefix: u8, is_private: bool) -> Ipv4Addr {
    loop {
        // Generate a random IP address with the first octet between 1 and 223.
        // Exclude 127 because it's reserved for loopback addresses.
        let first_octet = random::gen_range(1..=223) as u8;
        if first_octet == 127 {
            continue; // Skip loopback addresses
        }

        let ip = Ipv4Addr::new(
            first_octet,
            random::gen_range(0..=255) as u8,
            random::gen_range(0..=255) as u8,
            random::gen_range(0..=255) as u8,
        );

        // Convert the IP address into a 32-bit integer.
        let ip_u32 = u32::from(ip);
        // Calculate the subnet mask as a 32-bit integer.
        let mask = u32::MAX << (32 - prefix);
        // Obtain the network address by applying the mask.
        let network_u32 = ip_u32 & mask;
        let network_ip = Ipv4Addr::from(network_u32);

        if is_private && !is_private_ip(network_ip) {
            continue;
        }
        if !is_private && is_private_ip(network_ip) {
            continue;
        }

        return network_ip;
    }
}

// Rest of your functions remain the same
fn determine_ip_class(ip: &str) -> char {
    // Your existing implementation
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
    'A' // Fallback
}

pub fn is_valid_ip(ip: &str) -> bool {
    // Your existing implementation
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

fn is_private_ip(ip: Ipv4Addr) -> bool {
    // Your existing implementation
    match ip.octets() {
        [10, _, _, _] => true,
        [172, second, _, _] if (16..=31).contains(&second) => true,
        [192, 168, _, _] => true,
        _ => false,
    }
}

fn determine_subnets(ip: Ipv4Addr, prefix: u8, wanted_networks: u32) -> Vec<Ipv4Addr> {
    // Your existing implementation
    let mut subnets = Vec::new();
    let mask = u32::MAX << (32 - prefix);
    let network = u32::from(ip) & mask;

    let mut current_network = network;
    let step = 2u32.pow(32 - prefix as u32);
    for _ in 0..wanted_networks {
        subnets.push(Ipv4Addr::from(current_network));
        current_network += step;
    }

    subnets
}
