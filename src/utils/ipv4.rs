use std::net::Ipv4Addr;

use crate::models::Question;
use crate::utils::io::first_and_last_three;
use rand::thread_rng;
use rand::Rng;

pub fn generate_subnetting_question() -> Question {
    let mut rng = rand::thread_rng();

    // Generate a random prefix length between 24 and 30
    let prefix = rng.gen_range(24..=30);

    // Generate a random private network IP address
    let network_ip = generate_random_network_ip(&mut rng, prefix, false);

    // Create question text
    let min_networks = 6;
    let mut max_networks = 2u32.pow(32 - prefix as u32);
    if max_networks < min_networks {
        max_networks = min_networks;
    }
    let wanted_networks = rng.gen_range(min_networks..=max_networks);

    let question_text = format!(
        "Bilde {} neue Netze aus dem folgenden Netzwerk:\n {}/{}",
        wanted_networks, network_ip, prefix,
    );

    // Create answer options TODO:
    let options = vec![];

    // Determine correct answer TODO: Logik für den check der Antwort da alle antworten richtig
    // sein müssen und mit der derzueitigen Logik nicht möglich ist

    Question::new(question_text, options, 0)
}

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
        0 => rng.gen_range(0..127),   // Class A
        1 => rng.gen_range(128..191), // Class B
        2 => rng.gen_range(192..223), // Class C
        3 => rng.gen_range(224..239), // Class D
        4 => rng.gen_range(240..255), // Class E
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

fn generate_random_network_ip(rng: &mut impl Rng, prefix: u8, is_private: bool) -> Ipv4Addr {
    loop {
        // Generate a random IP address with the first octet between 1 and 223.
        // We exclude 127 because it's reserved for loopback addresses.
        let first_octet = rng.gen_range(1..224);
        if first_octet == 127 {
            continue; // Skip loopback addresses
        }
        let ip = Ipv4Addr::new(
            first_octet,
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        );

        // Convert the IP address into a 32-bit integer.
        let ip_u32 = u32::from(ip);
        // Calculate the subnet mask as a 32-bit integer.
        let mask = u32::MAX << (32 - prefix);
        // Obtain the network address by applying the mask (i.e., setting the host bits to 0).
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

fn is_private_ip(ip: Ipv4Addr) -> bool {
    match ip.octets() {
        [10, _, _, _] => true,
        [172, second, _, _] if (16..=31).contains(&second) => true,
        [192, 168, _, _] => true,
        _ => false,
    }
}

fn determine_subnets(ip: Ipv4Addr, prefix: u8, wanted_networks: u32) -> Vec<Ipv4Addr> {
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

#[cfg(test)]
mod tests {
    use super::*;
    /// Test that the function generates a valid IPv4 address within the expected range.
    #[test]
    fn test_generate_random_network_ip_valid_range() {
        let mut rng = thread_rng();
        let prefix = 26; // Example prefix
        let ip = generate_random_network_ip(&mut rng, prefix, false);

        let first_octet = ip.octets()[0];

        // Ensure that the IP is within valid public or private ranges (not loopback, multicast, etc.)
        assert!(
            first_octet != 127,
            "Generated IP should not be a loopback address"
        );
        assert!(
            first_octet < 224,
            "Generated IP should not be a multicast address"
        );
    }

    /// Test that the generated IP is aligned to the network address for the given prefix.
    #[test]
    fn test_generate_random_network_ip_alignment() {
        let mut rng = thread_rng();
        let prefix = 26;
        let ip = generate_random_network_ip(&mut rng, prefix, false);

        let ip_u32 = u32::from(ip);
        let mask = u32::MAX << (32 - prefix);
        let expected_network = ip_u32 & mask;

        assert_eq!(
            ip_u32, expected_network,
            "Generated IP is not correctly aligned to the subnet mask"
        );
    }

    /// Test that the function does not generate any loopback addresses.
    #[test]
    fn test_generate_random_network_ip_no_loopback() {
        let mut rng = thread_rng();
        let prefix = 26;
        for _ in 0..100 {
            let ip = generate_random_network_ip(&mut rng, prefix, false);
            assert_ne!(
                ip.octets()[0],
                127,
                "Generated IP should not be a loopback address"
            );
        }
    }

    /// Test that the function does not generate multicast or reserved addresses.
    #[test]
    fn test_generate_random_network_ip_no_multicast_or_reserved() {
        let mut rng = thread_rng();
        let prefix = 26;
        for _ in 0..100 {
            let ip = generate_random_network_ip(&mut rng, prefix, false);
            let first_octet = ip.octets()[0];

            assert!(
                !(224..=255).contains(&first_octet),
                "Generated IP should not be in the multicast or reserved range"
            );
        }
    }

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
    #[test]
    fn test_is_private_ip() {
        assert!(is_private_ip(Ipv4Addr::new(10, 0, 0, 1))); // 10.0.0.0/8
        assert!(is_private_ip(Ipv4Addr::new(172, 16, 0, 1))); // 172.16.0.0/12
        assert!(is_private_ip(Ipv4Addr::new(172, 31, 255, 255))); // Edge of 172.16.0.0/12
        assert!(is_private_ip(Ipv4Addr::new(192, 168, 1, 1))); // 192.168.0.0/16

        assert!(!is_private_ip(Ipv4Addr::new(8, 8, 8, 8))); // Public IP (Google DNS)
        assert!(!is_private_ip(Ipv4Addr::new(1, 1, 1, 1))); // Public IP (Cloudflare DNS)
        assert!(!is_private_ip(Ipv4Addr::new(100, 64, 0, 1))); // CGNAT (not private in this context)
    }

    #[test]
    fn test_generate_random_network_ip_private() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let ip = generate_random_network_ip(&mut rng, 24, true);
            assert!(
                is_private_ip(ip),
                "Generated private IP is not in private range: {}",
                ip
            );
        }
    }

    #[test]
    fn test_generate_random_network_ip_public() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let ip = generate_random_network_ip(&mut rng, 24, false);
            assert!(
                !is_private_ip(ip),
                "Generated public IP is in private range: {}",
                ip
            );
        }
    }

    #[test]
    fn test_generate_random_network_ip_subnet_alignment() {
        let mut rng = thread_rng();
        let prefix = 26; // Example subnet mask
        for _ in 0..100 {
            let ip = generate_random_network_ip(&mut rng, prefix, false);
            let ip_u32 = u32::from(ip);
            let mask = u32::MAX << (32 - prefix);
            let network_u32 = ip_u32 & mask;
            assert_eq!(
                ip,
                Ipv4Addr::from(network_u32),
                "Generated IP is not a valid network address"
            );
        }
    }

    #[test]
    fn test_single_subnet() {
        let ip = Ipv4Addr::new(192, 168, 1, 0);
        let subnets = determine_subnets(ip, 24, 1);
        assert_eq!(subnets, vec![Ipv4Addr::new(192, 168, 1, 0)]);
    }

    #[test]
    fn test_multiple_subnets() {
        let ip = Ipv4Addr::new(192, 168, 1, 0);
        let subnets = determine_subnets(ip, 24, 3);
        assert_eq!(
            subnets,
            vec![
                Ipv4Addr::new(192, 168, 1, 0),
                Ipv4Addr::new(192, 168, 2, 0),
                Ipv4Addr::new(192, 168, 3, 0),
            ]
        );
    }

    #[test]
    fn test_smaller_prefix() {
        let ip = Ipv4Addr::new(10, 0, 0, 0);
        let subnets = determine_subnets(ip, 16, 2);
        assert_eq!(
            subnets,
            vec![Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 1, 0, 0),]
        );
    }

    #[test]
    fn test_edge_case_max_networks() {
        let ip = Ipv4Addr::new(172, 16, 0, 0);
        let subnets = determine_subnets(ip, 30, 4);
        assert_eq!(
            subnets,
            vec![
                Ipv4Addr::new(172, 16, 0, 0),
                Ipv4Addr::new(172, 16, 0, 4),
                Ipv4Addr::new(172, 16, 0, 8),
                Ipv4Addr::new(172, 16, 0, 12),
            ]
        );
    }

    #[test]
    fn test_first_and_last_three_subnets() {
        let ip = Ipv4Addr::new(172, 16, 0, 0);
        let subnets = determine_subnets(ip, 25, 80);
        let result = first_and_last_three(&subnets);
        assert_eq!(
            result,
            vec![
                Ipv4Addr::new(172, 16, 0, 0),
                Ipv4Addr::new(172, 16, 0, 128),
                Ipv4Addr::new(172, 16, 1, 0),
                Ipv4Addr::new(172, 16, 38, 128),
                Ipv4Addr::new(172, 16, 39, 0),
                Ipv4Addr::new(172, 16, 39, 128),
            ]
        );
    }
}
