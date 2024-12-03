use rand::Rng;
use std::net::Ipv4Addr;
use crate::question::Question;


// A function that creates a random IPv4 address
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

// A function that determines the class of an IPv4 address
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

// A function that converts the class of an IPv4 address to a string
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
 
// A function that generates a question about the class of an IPv4 address
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
// A function that converts CIDR notation to a subnet mask
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
// A function that creates a random IP and SNM
pub fn create_ipv4_and_snm() -> (Ipv4Addr, u8) {
    let mut rng = rand::thread_rng();


    let ip = Ipv4Addr::new(
        rng.gen_range(1..=223), 
        rng.gen_range(0..=255), 
        rng.gen_range(0..=255), 
        rng.gen_range(1..=254)
    );

    let snm = rng.gen_range(8..=30);

    (ip, snm)

}

// A function that calculates the network ID
pub fn calculate_network_id(ip: Ipv4Addr, prefix: u8) -> Ipv4Addr {
    let mask = (0xFFFFFFFFu32 << (32 - prefix)) & 0xFFFFFFFF;

    let ip_u32: u32 = ip.into();

    let network_id_u32 = ip_u32 & mask;

    Ipv4Addr::from(network_id_u32)
}
// A function that gives out the number of available networks
pub fn available_networks(ip: Ipv4Addr, prefix: u8) -> u32 {
    let available_networks = 2u32.pow((32 - prefix).into()) - 2;

    available_networks
}

// A funcion that gives out the octet that needs to be changed for subnetting
pub fn actionbyte(snm: Ipv4Addr) -> u8 {
    let snm = snm.octets();
    let mut actionbyte = 0;
    for octet in snm.iter() {
        if *octet != 255 {
            break;
        }
        actionbyte += 1;
    }
    actionbyte
}

// A function that gives out the number of subnets that can be created
pub fn available_subnets(snm: Ipv4Addr) -> u32 {
    let actionbyte = actionbyte(snm);
    let available_subnets = 2u32.pow((8 - actionbyte).into()) - 2;
    available_subnets
}

// A funcion calculates the subnet increment
pub fn subnet_increment(snm: Ipv4Addr) -> u32 {
    let actionbyte = actionbyte(snm);
    let subnet_increment = 2u32.pow((8 - actionbyte).into());
    subnet_increment
}


// A function that takes an ip and cidr snm and a number of subnets and returns the new snm in cidr
// notation
pub fn new_snm(ip: Ipv4Addr, prefix: u8, subnets: u32) -> u8 {
    let available_subnets = available_subnets(ip);
    let subnet_increment = subnet_increment(ip);
    let new_prefix = prefix + (32 - prefix).saturating_sub(subnets.trailing_zeros());
    new_prefix
}

// 
