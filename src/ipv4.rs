use rand::Rng;
use std::net::Ipv4Addr;

use crate::question::Question;



pub fn create_ipv4() -> Ipv4Addr {
    let mut rng = rand::thread_rng();
    let ipv4 = Ipv4Addr::new(
        rng.gen_range(1..=255), 
        rng.gen_range(0..=255), 
        rng.gen_range(0..=255), 
        rng.gen_range(0..=255)
    );
    ipv4    
}


pub fn ipv4_range(ipv4: Ipv4Addr) -> i8 {
    let first_octet = ipv4.octets()[0];

    match first_octet {
        0..=127   => 0, // Class A
        128..=191 => 1, // Class B
        192..=223 => 2, // Class C
        224..=239 => 3, // Class D (Multicast)
        240..=255 => 4, // Class E (Experimental/Reserved)
    }
}


pub fn _range_to_str(class: i8) -> &'static str {
    match class {
    0 => "Klasse A",
    1 => "Klasse B", 
    2 => "Klasse C",
    3 => "Klasse D",
    4 => "Klasse E",
    _ => "Ungültige Klasse"
    }
}
 

pub fn generate_ipv4_question() -> Question {
    let ipv4 = create_ipv4();

    let correct_answer = ipv4_range(ipv4);


    let options = vec![
        "Klasse A".to_string(),
        "Klasse B".to_string(),
        "Klasse C".to_string(),
        "Klasse D".to_string(),
        "Klasse E".to_string(),
    ];

    let question_text = format!("In welche Klasse befindet sich diese IPV4 Addresse: {}", ipv4);

    Question::new(question_text, options, correct_answer as usize )


}

pub fn convert_snm(cidr: u8) -> Ipv4Addr {
    // take xFFFFFFFF and shift bits according to cidr notation
    let mask = (0xFFFFFFFFu32 << (32 - cidr)) & 0xFFFFFFFF;
    // Create the windows SNM 
    let snm = Ipv4Addr::new(
        ((mask >> 24) & 0xFF) as u8, 
        ((mask >> 16) & 0xFF) as u8, 
        ((mask >> 8) & 0xFF) as u8, 
        (mask & 0xFF) as u8,
    );
    snm
}
