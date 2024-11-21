use rand::Rng;
use std::net::Ipv4Addr;



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


pub fn range_to_str(class: i8) -> &'static str {
    match class {
    0 => "Klasse A",
    1 => "Klasse B", 
    2 => "Klasse C",
    3 => "Klasse D",
    4 => "Klasse E",
    _ => "Ungültige Klasse"
    }
}
