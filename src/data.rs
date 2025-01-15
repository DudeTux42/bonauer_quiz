use crate::category::Category;
use crate::ipv4::generate_ipv4_question;
use crate::question::Question;
use crate::quiz::Quiz;

pub fn create_sample_quiz() -> Quiz {
    let mut quiz = Quiz::new();

    // Category: Math
    let mut math_category = Category::new("Mathematics".to_string());
    math_category.add_question(Question::new(
        "What is 2 + 2?".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
        3,
    ));
    math_category.add_question(Question::new(
        "What is 3 * 3?".to_string(),
        vec!["6".to_string(), "9".to_string(), "12".to_string()],
        1,
    ));
    math_category.add_question(Question::new(
        "What is 12 / 4?".to_string(),
        vec!["2".to_string(), "3".to_string(), "4".to_string()],
        1,
    ));

    



    // Category: IT
    let mut it_category = Category::new("IT".to_string());
    it_category.add_question(Question::new(
        "What is the primary language used for web development?".to_string(),
        vec!["Java".to_string(), "JavaScript".to_string(), "C++".to_string()],
        1,
    ));

    it_category.add_question(Question::new(
        "Who is known as the father of Free Software Foundation?".to_string(),
        vec![
            "Richard Mathew Stallman".to_string(), // Correct
            "Linus Torvalds".to_string(), // Hard false
            "Ken Thompson".to_string(), // Hard false
            "Dennis Ritchie".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "In which year was the ‘C’ programming language developed?".to_string(),
        vec![
            "1972".to_string(), // Correct
            "1965".to_string(), // Hard false
            "1980".to_string(), // Hard false
            "1990".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who is known as the father of Internet?".to_string(),
        vec![
            "Vinton Cerf".to_string(), // Correct
            "Tim Berners-Lee".to_string(), // Hard false
            "Marc Andreessen".to_string(), // Hard false
            "Robert Kahn".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who used the binary system of numeration for the first time?".to_string(),
        vec![
            "Thomas Harriot".to_string(), // Correct
            "John von Neumann".to_string(), // Hard false
            "Isaac Newton".to_string(), // Hard false
            "Charles Babbage".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who is the first computer programmer?".to_string(),
        vec![
            "Ada Lovelace".to_string(), // Correct
            "Grace Hopper".to_string(), // Hard false
            "John von Neumann".to_string(), // Hard false
            "Alan Turing".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "In which year was computer graphics originated?".to_string(),
        vec![
            "1940".to_string(), // Correct
            "1930".to_string(), // Hard false
            "1950".to_string(), // Hard false
            "1960".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who is the inventor of Supercomputer?".to_string(),
        vec![
            "Seymour Cray".to_string(), // Correct
            "Bill Gates".to_string(), // Hard false
            "Mark Zuckerberg".to_string(), // Hard false
            "Steve Jobs".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name of World’s first digital Computer?".to_string(),
        vec![
            "Mark 1".to_string(), // Correct
            "ENIAC".to_string(), // Hard false
            "Colossus".to_string(), // Hard false
            "Atanasoff-Berry Computer".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name of India’s first indigenous supercomputer developed by CDAC?".to_string(),
        vec![
            "Param".to_string(), // Correct
            "Brahmos".to_string(), // Hard false
            "Sahasra".to_string(), // Hard false
            "Vishal".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which was the first Pocket Computer?".to_string(),
        vec![
            "SHARP PC1211".to_string(), // Correct
            "Casio fx-7000G".to_string(), // Hard false
            "HP-35".to_string(), // Hard false
            "TRS-80".to_string(), // Hard false
        ],
        0,
    ));
    

    it_category.add_question(Question::new(
        "What is the name of the tablet introduced by Amazon?".to_string(),
        vec![
            "Kindle Fire".to_string(), // Correct
            "Nook Tablet".to_string(), // Hard false
            "Surface Pro".to_string(), // Hard false
            "iPad Mini".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who invented the Computer Mouse?".to_string(),
        vec![
            "Douglas Engelbart".to_string(), // Correct
            "Alan Turing".to_string(), // Hard false
            "John von Neumann".to_string(), // Hard false
            "Bill Gates".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Speak to Tweet is a service given by which giant?".to_string(),
        vec![
            "Google".to_string(), // Correct
            "Twitter".to_string(), // Hard false
            "Microsoft".to_string(), // Hard false
            "Facebook".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Expand HDMI".to_string(),
        vec![
            "High Definition Multimedia Interface".to_string(), // Correct
            "High Density Media Interface".to_string(), // Hard false
            "High Data Multimedia Integration".to_string(), // Hard false
            "High Definition Media Interconnect".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What was developed by PYRA Labs and later sold to Google?".to_string(),
        vec![
            "Blogger".to_string(), // Correct
            "YouTube".to_string(), // Hard false
            "Drive".to_string(), // Hard false
            "Chrome".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who is known as the father of the World Wide Web?".to_string(),
        vec![
            "Tim Berners-Lee".to_string(), // Correct
            "Vinton Cerf".to_string(), // Hard false
            "Robert Kahn".to_string(), // Hard false
            "Marc Andreessen".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which method is used to connect to a remote computer?".to_string(),
        vec![
            "VPN".to_string(),          // Plausible but incorrect
            "Telnet".to_string(),       // Outdated and insecure
            "File Transfer Protocol".to_string(), // Incorrect usage
            "Remote Desktop Protocol".to_string(), // Correct
        ],
        3,
    ));

    it_category.add_question(Question::new(
        "What do you call a single point on a computer screen?".to_string(),
        vec![
            "Pixel".to_string(), // Correct
            "Bit".to_string(), // Hard false
            "Dot".to_string(), // Hard false
            "Node".to_string(), // Hard false
        ],
        0,
    ));
    
    
    it_category.add_question(Question::new(
        "The CPU chip used in a computer is made of which element?".to_string(),
        vec![
            "Silica".to_string(), // Correct
            "Copper".to_string(), // Hard false
            "Graphite".to_string(), // Hard false
            "Plastic".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which computer was designed to be as compact as possible?".to_string(),
        vec![
            "Micro Computer".to_string(), // Correct
            "Super Computer".to_string(), // Hard false
            "Mainframe".to_string(), // Hard false
            "Workstation".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name of the device which produces hard copy graphics?".to_string(),
        vec![
            "Plotter".to_string(), // Correct
            "Printer".to_string(), // Hard false
            "Scanner".to_string(), // Hard false
            "Copier".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "When is World Computer Literacy Day celebrated?".to_string(),
        vec![
            "December 2".to_string(), // Correct
            "January 15".to_string(), // Hard false
            "June 10".to_string(), // Hard false
            "September 25".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "The processing power of a CPU is measured in?".to_string(),
        vec![
            "MIPS".to_string(), // Correct
            "GHz".to_string(), // Hard false
            "Bits".to_string(), // Hard false
            "Cores".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Windows 10 was released on which day?".to_string(),
        vec![
            "July 29".to_string(), // Correct
            "August 15".to_string(), // Hard false
            "October 5".to_string(), // Hard false
            "September 1".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which command is used to view the sub-directory structure of a drive?".to_string(),
        vec![
            "TREE".to_string(), // Correct
            "DIR".to_string(), // Hard false
            "LS".to_string(), // Hard false
            "FIND".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Round Robin scheduling is the preemptive version of?".to_string(),
        vec![
            "FIFO".to_string(), // Correct
            "LIFO".to_string(), // Hard false
            "SJF".to_string(), // Hard false
            "Priority".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "When did IBM release its first version of DOS OS 1.0?".to_string(),
        vec![
            "1981".to_string(), // Correct
            "1980".to_string(), // Hard false
            "1982".to_string(), // Hard false
            "1979".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "The Start button was introduced in which operating system?".to_string(),
        vec![
            "Windows 95".to_string(), // Correct
            "Windows 98".to_string(), // Hard false
            "Windows XP".to_string(), // Hard false
            "Windows ME".to_string(), // Hard false
        ],
        0,
    ));



    it_category.add_question(Question::new(
        "Which system software does the job of merging the records from two files to one?".to_string(),
        vec![
            "Database Mangement System (DBMS)".to_string(), // Correct
            "Integrated Development Environment".to_string(), // Hard false
            "File Manager".to_string(), // Hard false
            "Compiler".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "While running DOS on a computer, which command is used to duplicate the entire diskette?".to_string(),
        vec![
            "DISKCOPY".to_string(), // Correct
            "COPY".to_string(), // Hard false
            "XCOPY".to_string(), // Hard false
            "FORMAT".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name given to the organized collection of software that controls the entire operation of a computer?".to_string(),
        vec![
            "Operating System".to_string(), // Correct
            "Application Software".to_string(), // Hard false
            "Device Driver".to_string(), // Hard false
            "BIOS".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name of the OS for the laptop MACLITE?".to_string(),
        vec![
            "OZ".to_string(), // Correct
            "MACOS".to_string(), // Hard false
            "LiteOS".to_string(), // Hard false
            "Catalina".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the memory range from 1k-640k called?".to_string(),
        vec![
            "Conventional memory".to_string(), // Correct
            "Virtual memory".to_string(), // Hard false
            "Extended memory".to_string(), // Hard false
            "Cache memory".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "IBM released its first PC in 1981, what was the name of the OS that was popular at that time?".to_string(),
        vec![
            "CP/M".to_string(), // Correct
            "DOS".to_string(), // Hard false
            "UNIX".to_string(), // Hard false
            "Windows 1.0".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "How long is an IPv6 Address?".to_string(),
        vec![
            "128 bit (16 byte)".to_string(), // Correct
            "64 bit (8 byte)".to_string(), // Hard false
            "32 bit (4 byte)".to_string(), // Hard false
            "256 bit (32 byte)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol does DHCP use at the transport layer?".to_string(),
        vec![
            "UDP".to_string(), // Correct
            "TCP".to_string(), // Hard false
            "ICMP".to_string(), // Hard false
            "HTTP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol is used to send a destination network unknown message back to the originating host?".to_string(),
        vec![
            "ICMP".to_string(), // Correct
            "ARP".to_string(), // Hard false
            "IP".to_string(), // Hard false
            "SMTP".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "Which WLAN IEEE Specification allows up to 54 Mbps at 2.4 GHz?".to_string(),
        vec![
            "IEEE 802.11G".to_string(), // Correct
            "IEEE 802.11B".to_string(), // Hard false
            "IEEE 802.11N".to_string(), // Hard false
            "IEEE 802.11A".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol does PPP use to identify the network layer protocol?".to_string(),
        vec![
            "NCP".to_string(), // Correct
            "LCP".to_string(), // Hard false
            "ICMP".to_string(), // Hard false
            "DHCP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "HBA stands for?".to_string(),
        vec![
            "Host Bus Adapter".to_string(), // Correct
            "Hyper Bus Array".to_string(), // Hard false
            "Hardware Base Adapter".to_string(), // Hard false
            "High Bandwidth Access".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the second generation of the Web called?".to_string(),
        vec![
            "Web 2.0".to_string(), // Correct
            "Web 1.0".to_string(), // Hard false
            "Semantic Web".to_string(), // Hard false
            "Dynamic Web".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "If you have a CISCO Mesh network, what protocol allows multiple APs to connect with many redundant connections between nodes?".to_string(),
        vec![
            "AWPP".to_string(), // Correct
            "BGP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
            "RIP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol is used to identify the hardware address of a local device?".to_string(),
        vec![
            "Address Resolution Protocol".to_string(), // Correct
            "Dynamic Host Configuration Protocol".to_string(), // Hard false
            "Internet Control Message Protocol".to_string(), // Hard false
            "Transmission Control Protocol".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "PAT Address Translation is also termed as what?".to_string(),
        vec![
            "NAT Overload".to_string(), // Correct
            "Static NAT".to_string(), // Hard false
            "Dynamic NAT".to_string(), // Hard false
            "Proxy ARP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "When was the term Social Networking first used?".to_string(),
        vec![
            "1954".to_string(), // Correct
            "1994".to_string(), // Hard false
            "2004".to_string(), // Hard false
            "1974".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who founded 'MySpace'?".to_string(),
        vec![
            "Tom Anderson".to_string(), // Correct
            "Mark Zuckerberg".to_string(), // Hard false
            "Jack Dorsey".to_string(), // Hard false
            "Evan Spiegel".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What type of audience are primarily in a social network?".to_string(),
        vec![
            "Joiners".to_string(), // Correct
            "Creators".to_string(), // Hard false
            "Critics".to_string(), // Hard false
            "Spectators".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Twitter is an example of what service?".to_string(),
        vec![
            "Micro Blogging".to_string(), // Correct
            "Social Networking".to_string(), // Hard false
            "News Aggregation".to_string(), // Hard false
            "Instant Messaging".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the character limit for a tweet in Twitter?".to_string(),
        vec![
            "280".to_string(), // Correct 
            "140".to_string(), // Hard false
            "160".to_string(), // Hard false
            "200".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What was the largest social network prior to Facebook?".to_string(),
        vec![
            "MySpace".to_string(), // Correct
            "Orkut".to_string(), // Hard false
            "LinkedIn".to_string(), // Hard false
            "Hi5".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "When did social networking first become popular online?".to_string(),
        vec![
            "2003".to_string(), // Correct
            "2005".to_string(), // Hard false
            "2000".to_string(), // Hard false
            "1999".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is YouTube’s slogan?".to_string(),
        vec![
            "Broadcast Yourself".to_string(), // Correct
            "Stream the World".to_string(), // Hard false
            "Watch and Share".to_string(), // Hard false
            "Create and Connect".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "P2P, B2B, and B2C are part of?".to_string(),
        vec![
            "Share Economy".to_string(), // Correct
            "Cryptocurrency".to_string(), // Hard false
            "Online Marketplaces".to_string(), // Hard false
            "Blockchain Technology".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Winchester drive is also called as what?".to_string(),
        vec![
            "Hard disk drive".to_string(), // Correct
            "Solid-State Drive".to_string(), // Hard false
            "Floppy Drive".to_string(), // Hard false
            "Tape Drive".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "What kind of connectors are used to connect a PC power supply to a hardware?".to_string(),
        vec![
            "Molex".to_string(), // Correct
            "SATA".to_string(), // Hard false
            "PCIe".to_string(), // Hard false
            "USB".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the term ‘Wave Table Synthesis’ related to?".to_string(),
        vec![
            "Sound".to_string(), // Correct
            "Graphics".to_string(), // Hard false
            "Networking".to_string(), // Hard false
            "Storage".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What type of memory is Pendrive?".to_string(),
        vec![
            "Flash Memory".to_string(), // Correct
            "RAM".to_string(), // Hard false
            "ROM".to_string(), // Hard false
            "Cache".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which IRQ does the hard disk drive use?".to_string(),
        vec![
            "14".to_string(), // Correct
            "10".to_string(), // Hard false
            "5".to_string(), // Hard false
            "7".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Who invented Optical Disc Media?".to_string(),
        vec![
            "James Russel".to_string(), // Correct
            "Philips".to_string(), // Hard false
            "Sony".to_string(), // Hard false
            "George Eastman".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is ‘Arrandale’ code name for?".to_string(),
        vec![
            "Intel Processor".to_string(), // Correct
            "AMD Graphics Card".to_string(), // Hard false
            "NVIDIA Chipset".to_string(), // Hard false
            "Apple M1 Chip".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What hardware was used by the initial generation of computers?".to_string(),
        vec![
            "Valves".to_string(), // Correct
            "Transistors".to_string(), // Hard false
            "Microchips".to_string(), // Hard false
            "Integrated Circuits".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which was the first computer made available for commercial use?".to_string(),
        vec![
            "UNIVAC".to_string(), // Correct
            "ENIAC".to_string(), // Hard false
            "IBM PC".to_string(), // Hard false
            "Altair 8800".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Name the first mechanical computer designed by Charles Babbage called?".to_string(),
        vec![
            "Analytical Engine".to_string(), // Correct
            "Difference Engine".to_string(), // Hard false
            "Turing Machine".to_string(), // Hard false
            "Zuse Z3".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "The concentric circles on the platter of a hard disk are known as?".to_string(),
        vec![
            "Tracks".to_string(), // Correct
            "Sectors".to_string(), // Hard false
            "Clusters".to_string(), // Hard false
            "Cylinders".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "IRQ6 is commonly assigned to?".to_string(),
        vec![
            "Floppy Drive Controller".to_string(), // Correct
            "Hard Disk Controller".to_string(), // Hard false
            "Sound Card".to_string(), // Hard false
            "Network Adapter".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which component in a PC regulates the color depth and screen resolution of a monitor?".to_string(),
        vec![
            "VRAM".to_string(), // Correct
            "GPU".to_string(), // Hard false
            "Monitor".to_string(), // Hard false
            "RAM".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "A Computer programming language for simulating models of business activity is?".to_string(),
        vec![
            "GPSS".to_string(), // Correct
            "COBOL".to_string(), // Hard false
            "BASIC".to_string(), // Hard false
            "Simula".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "The words that are set aside by the programming language for its own use are called as what?".to_string(),
        vec![
            "Control Structures".to_string(), // Correct
            "Variables".to_string(), // Hard false
            "Functions".to_string(), // Hard false
            "Classes".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which programming language is used for scientific calculations?".to_string(),
        vec![
            "FORTRAN".to_string(), // Correct
            "MATLAB".to_string(), // Hard false
            "C".to_string(), // Hard false
            "Python".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which JavaScript framework is most commonly used for modern web development?".to_string(),
        vec![
            "React".to_string(),    // Correct
            "Svelte".to_string(),   // Modern, but less common
            "Ember.js".to_string(), // Still used, but outdated compared to React
            "Backbone.js".to_string(), // Very outdated, not commonly used anymore
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which language is used for the development of various games?".to_string(),
        vec![
            "C++".to_string(), // Correct
            "Python".to_string(), // Hard false
            "JavaScript".to_string(), // Hard false
            "Rust".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which language was devised by Dr. Seymour Papert?".to_string(),
        vec![
            "LOGO".to_string(), // Correct
            "Pascal".to_string(), // Hard false
            "LISP".to_string(), // Hard false
            "COBOL".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which computer language is used for Artificial Intelligence?".to_string(),
        vec![
            "Prolog".to_string(), // Correct
            "Python".to_string(), // Hard false
            "LISP".to_string(), // Hard false
            "Java".to_string(), // Hard false
        ],
        1,
    ));

    it_category.add_question(Question::new(
        "Who is the creator of the Pascal language?".to_string(),
        vec![
            "Niclaus Wirth".to_string(), // Correct
            "Dennis Ritchie".to_string(), // Hard false
            "Bjarne Stroustrup".to_string(), // Hard false
            "James Gosling".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "Which systems programming environment is commonly used for microcontrollers like ESP32?".to_string(),
        vec![
            "C++".to_string(),   // Correct
            "Python".to_string(), // Plausible, used for simpler applications
            "JavaScript".to_string(), // Used with frameworks like NodeMCU for microcontrollers
            "Rust".to_string(),  // Increasingly popular, but less common than C++ for embedded systems
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which programming language is known for supporting structured programming with modern features?".to_string(),
        vec![
            "Rust".to_string(),      // Correct
            "C".to_string(),         // Plausible, classic for structured programming
            "Python".to_string(),    // Plausible, supports structured programming
            "JavaScript".to_string(),// Plausible, used in structured codebases
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "MS/DOS is written in which language?".to_string(),
        vec![
            "C++".to_string(), // Correct
            "Assembly".to_string(), // Hard false
            "BASIC".to_string(), // Hard false
            "C".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "A program element that allows structuring of a program in a different way is called?".to_string(),
        vec![
            "Co-Routine".to_string(), // Correct
            "Sub-Routine".to_string(), // Hard false
            "Function".to_string(), // Hard false
            "Loop".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "A name given by Intel to high-speed MOS technology is called?".to_string(),
        vec![
            "HMOS".to_string(), // Correct
            "CMOS".to_string(), // Hard false
            "BIOS".to_string(), // Hard false
            "NMOS".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which country recently shut down the use of FM Radio?".to_string(),
        vec![
            "Norway".to_string(), // Correct
            "Sweden".to_string(), // Hard false
            "Finland".to_string(), // Hard false
            "Denmark".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does OSI stand for?".to_string(),
        vec![
            "Open Systems Interconnection (Layer 7)".to_string(), // Correct
            "Open Source Interface (Layer 7)".to_string(), // Hard false
            "Online Systems Integration (Layer 7)".to_string(), // Hard false
            "Open Secure Interface (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which OSI layer is responsible for routing data between different networks?".to_string(),
        vec![
            "Network Layer (Layer 3)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which layer of the OSI model is responsible for establishing, maintaining, and terminating communication sessions?".to_string(),
        vec![
            "Session Layer (Layer 5)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Presentation Layer (Layer 6)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which OSI layer handles the physical transmission of data over a medium?".to_string(),
        vec![
            "Physical Layer (Layer 1)".to_string(), // Correct
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which OSI layer is responsible for error detection and correction?".to_string(),
        vec![
            "Data Link Layer (Layer 2)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which OSI layer formats data and is responsible for data encryption and compression?".to_string(),
        vec![
            "Presentation Layer (Layer 6)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Session Layer (Layer 5)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "At which layer of the OSI model do HTTP, FTP, and SMTP operate?".to_string(),
        vec![
            "Application Layer (Layer 7)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which OSI layer is responsible for end-to-end communication and flow control?".to_string(),
        vec![
            "Transport Layer (Layer 4)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Physical Layer (Layer 1)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "In the OSI model, which layer is responsible for IP addressing?".to_string(),
        vec![
            "Network Layer (Layer 3)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following layers of the OSI model is responsible for converting digital signals to analog and vice versa?".to_string(),
        vec![
            "Physical Layer (Layer 1)".to_string(), // Correct
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Presentation Layer (Layer 6)".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "What is the size (in bits) of an IPv4 address?".to_string(),
        vec![
            "32 bits".to_string(), // Correct
            "64 bits".to_string(), // Hard false
            "128 bits".to_string(), // Hard false
            "16 bits".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which year was IPv6 officially released?".to_string(),
        vec![
            "1998".to_string(), // Correct
            "2001".to_string(), // Hard false
            "1994".to_string(), // Hard false
            "2005".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which IPv4 class is used for private addresses like 192.168.x.x?".to_string(),
        vec![
            "Class C".to_string(), // Correct
            "Class A".to_string(), // Hard false
            "Class B".to_string(), // Hard false
            "Class D".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the maximum number of hosts that can be supported on a subnet with a /24 prefix in IPv4?".to_string(),
        vec![
            "254 hosts".to_string(), // Correct
            "256 hosts".to_string(), // Hard false
            "128 hosts".to_string(), // Hard false
            "512 hosts".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the total number of possible IPv4 addresses?".to_string(),
        vec![
            "4.3 billion".to_string(), // Correct
            "2.1 billion".to_string(), // Hard false
            "8.6 billion".to_string(), // Hard false
            "1.5 billion".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is a valid IPv6 address format?".to_string(),
        vec![
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(), // Correct
            "192.168.0.1".to_string(), // Hard false
            "255.255.255.0".to_string(), // Hard false
            "3000:0db8:85a3:0000::0000".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which part of an IPv4 address represents the network portion?".to_string(),
        vec![
            "The first few bits, determined by the subnet mask".to_string(), // Correct
            "The last few bits, after the slash".to_string(), // Hard false
            "All of the address".to_string(), // Hard false
            "The middle section of the address".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does the IPv6 address block 2001:0db8::/32 represent?".to_string(),
        vec![
            "It is reserved for documentation purposes".to_string(), // Correct
            "It is used for local communication only".to_string(), // Hard false
            "It is a private network address".to_string(), // Hard false
            "It is the address for the default gateway".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does the CIDR notation /8 represent in terms of IPv4 subnetting?".to_string(),
        vec![
            "It represents 16,777,216 possible addresses".to_string(), // Correct
            "It represents 256 possible addresses".to_string(), // Hard false
            "It represents 1,024 possible addresses".to_string(), // Hard false
            "It represents 65,536 possible addresses".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which IPv6 feature helps avoid network address translation (NAT)?".to_string(),
        vec![
            "Auto-configuration (Stateless Address Autoconfiguration)".to_string(), // Correct
            "Private addressing".to_string(), // Hard false
            "IP Masquerading".to_string(), // Hard false
            "Dynamic Host Configuration Protocol (DHCP)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which type of address is used to identify a single device in IPv6?".to_string(),
        vec![
            "Unicast".to_string(), // Correct
            "Multicast".to_string(), // Hard false
            "Anycast".to_string(), // Hard false
            "Broadcast".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the purpose of the IPv6 link-local address?".to_string(),
        vec![
            "To allow devices on the same link to communicate".to_string(), // Correct
            "To identify a device across the entire network".to_string(), // Hard false
            "To enable NAT traversal".to_string(), // Hard false
            "To route packets across the internet".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which subnet mask is equivalent to the IPv4 CIDR prefix /16?".to_string(),
        vec![
            "255.255.0.0".to_string(), // Correct
            "255.255.255.0".to_string(), // Hard false
            "255.255.255.255".to_string(), // Hard false
            "255.255.255.224".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which IPv6 address is used to send a packet to multiple devices on a local network?".to_string(),
        vec![
            "Multicast address".to_string(), // Correct
            "Unicast address".to_string(), // Hard false
            "Anycast address".to_string(), // Hard false
            "Broadcast address".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the purpose of subnetting in IPv4 networking?".to_string(),
        vec![
            "To divide a large network into smaller, more manageable sub-networks".to_string(), // Correct
            "To increase the total number of available addresses".to_string(), // Hard false
            "To allow the use of private addresses only".to_string(), // Hard false
            "To combine multiple networks into one larger network".to_string(), // Hard false
        ],
        0,
    ));

 
    it_category.add_question(Question::new(
        "What is the length of a MAC address?".to_string(),
        vec![
            "48 bits".to_string(), // Correct
            "32 bits".to_string(), // Hard false
            "64 bits".to_string(), // Hard false
            "128 bits".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the format of a MAC address?".to_string(),
        vec![
            "6 pairs of hexadecimal digits".to_string(), // Correct
            "4 pairs of hexadecimal digits".to_string(), // Hard false
            "8 digits of binary numbers".to_string(), // Hard false
            "A series of numbers and letters separated by colons".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is true about a MAC address?".to_string(),
        vec![
            "It uniquely identifies network interfaces on a local network".to_string(), // Correct
            "It identifies the network protocol version".to_string(), // Hard false
            "It changes with each network connection".to_string(), // Hard false
            "It is used to route traffic on the internet".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which part of a MAC address identifies the manufacturer of the network interface?".to_string(),
        vec![
            "The first 3 bytes (OUI - Organizationally Unique Identifier)".to_string(), // Correct
            "The last 3 bytes".to_string(), // Hard false
            "The middle 2 bytes".to_string(), // Hard false
            "The entire address".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does MAC stand for in the context of networking?".to_string(),
        vec![
            "Media Access Control".to_string(), // Correct
            "Machine Access Code".to_string(), // Hard false
            "Modular Access Control".to_string(), // Hard false
            "Maximum Allowed Capacity".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which layer of the OSI model uses MAC addresses for communication?".to_string(),
        vec![
            "Data Link Layer (Layer 2)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Session Layer (Layer 5)".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is a valid MAC address format?".to_string(),
        vec![
            "00:1A:2B:3C:4D:5E".to_string(), // Correct
            "192.168.1.1".to_string(), // Hard false
            "00-1A-2B-3C-4D-5E".to_string(), // Hard false
            "0x001A2B3C4D5E".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is a common way to represent a MAC address?".to_string(),
        vec![
            "Hexadecimal format with colons or hyphens".to_string(), // Correct
            "Decimal format with spaces".to_string(), // Hard false
            "Binary format with dashes".to_string(), // Hard false
            "Octal format with commas".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Can a MAC address be changed or spoofed?".to_string(),
        vec![
            "Yes, it can be spoofed through software".to_string(), // Correct
            "No, a MAC address is permanent".to_string(), // Hard false
            "Yes, but only through hardware modifications".to_string(), // Hard false
            "No, the MAC address is assigned by the router".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the primary purpose of a MAC address in networking?".to_string(),
        vec![
            "To identify devices on a local network".to_string(), // Correct
            "To route packets between networks".to_string(), // Hard false
            "To secure network connections".to_string(), // Hard false
            "To determine the IP address of a device".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does it mean if two devices have the same MAC address?".to_string(),
        vec![
            "It may cause a conflict on the network".to_string(), // Correct
            "It means they are connected to different networks".to_string(), // Hard false
            "It is a sign of network security".to_string(), // Hard false
            "It is completely fine as long as they use different IP addresses".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the function of the MAC address in Ethernet frames?".to_string(),
        vec![
            "It is used for identifying the source and destination devices".to_string(), // Correct
            "It is used to define the protocol used in the frame".to_string(), // Hard false
            "It is used to assign an IP address to the frame".to_string(), // Hard false
            "It is used to encrypt the data within the frame".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is true about a MAC address in a Wi-Fi network?".to_string(),
        vec![
            "It is used for identifying the wireless device on the network".to_string(), // Correct
            "It is used for assigning a channel to the network".to_string(), // Hard false
            "It determines the Wi-Fi encryption type".to_string(), // Hard false
            "It is used for determining the frequency band".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "In which range does the OUI (Organizationally Unique Identifier) of a MAC address lie?".to_string(),
        vec![
            "First 3 bytes".to_string(), // Correct
            "Last 3 bytes".to_string(), // Hard false
            "Middle 2 bytes".to_string(), // Hard false
            "Entire 6 bytes".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the range of MAC addresses used for locally administered addresses?".to_string(),
        vec![
            "Addresses where the first bit of the first byte is set to 1".to_string(), // Correct
            "Addresses where the last byte is reserved".to_string(), // Hard false
            "Addresses where the first byte is set to 0".to_string(), // Hard false
            "Addresses that end with FF".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "Which of the following is used for wired Ethernet connections?".to_string(),
        vec![
            "RJ45".to_string(), // Correct
            "USB-C".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
            "Thunderbolt".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the maximum theoretical speed of Wi-Fi 6 (802.11ax)?".to_string(),
        vec![
            "9.6 Gbps".to_string(), // Correct
            "1 Gbps".to_string(), // Hard false
            "5 Gbps".to_string(), // Hard false
            "40 Gbps".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which Bluetooth version introduced support for Low Energy (LE)?".to_string(),
        vec![
            "Bluetooth 4.0".to_string(), // Correct
            "Bluetooth 3.0".to_string(), // Hard false
            "Bluetooth 2.0".to_string(), // Hard false
            "Bluetooth 5.0".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the main purpose of an RJ45 connector?".to_string(),
        vec![
            "It is used for Ethernet networking".to_string(), // Correct
            "It is used for HDMI connections".to_string(), // Hard false
            "It is used for USB connections".to_string(), // Hard false
            "It is used for audio equipment".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the typical range of Bluetooth 5.0?".to_string(),
        vec![
            "Up to 240 meters".to_string(), // Correct
            "Up to 10 meters".to_string(), // Hard false
            "Up to 100 meters".to_string(), // Hard false
            "Up to 1 meter".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which Wi-Fi standard operates on the 2.4 GHz and 5 GHz bands?".to_string(),
        vec![
            "802.11ac".to_string(), // Correct
            "802.11b".to_string(), // Hard false
            "802.11n".to_string(), // Hard false
            "802.11a".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following connectors is commonly used for fiber optic cables?".to_string(),
        vec![
            "SC Connector".to_string(), // Correct
            "RJ45".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
            "USB-A".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does the term 'Bluetooth pairing' refer to?".to_string(),
        vec![
            "Establishing a secure connection between Bluetooth devices".to_string(), // Correct
            "Charging Bluetooth devices".to_string(), // Hard false
            "Setting up a Wi-Fi network".to_string(), // Hard false
            "Updating the Bluetooth version on a device".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the frequency range of Wi-Fi 5 (802.11ac)?".to_string(),
        vec![
            "5 GHz".to_string(), // Correct
            "2.4 GHz".to_string(), // Hard false
            "900 MHz".to_string(), // Hard false
            "1.2 GHz".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which version of Bluetooth allows for faster data transfer rates of up to 3 Mbps?".to_string(),
        vec![
            "Bluetooth 2.0".to_string(), // Correct
            "Bluetooth 3.0".to_string(), // Hard false
            "Bluetooth 4.0".to_string(), // Hard false
            "Bluetooth 5.0".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which connection type is commonly used for long-distance, high-speed internet connections in homes and businesses?".to_string(),
        vec![
            "Fiber Optic".to_string(), // Correct
            "Wi-Fi".to_string(), // Hard false
            "Ethernet (RJ45)".to_string(), // Hard false
            "Bluetooth".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the maximum range of Wi-Fi 6 (802.11ax)?".to_string(),
        vec![
            "Up to 300 meters".to_string(), // Correct
            "Up to 100 meters".to_string(), // Hard false
            "Up to 500 meters".to_string(), // Hard false
            "Up to 50 meters".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is true about RJ45 connectors?".to_string(),
        vec![
            "Used for Ethernet networking over twisted pair cables".to_string(), // Correct
            "Used for HDMI video connections".to_string(), // Hard false
            "Used for connecting audio equipment".to_string(), // Hard false
            "Used for USB connections".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the name of the Wi-Fi technology that allows devices to communicate without a traditional router?".to_string(),
        vec![
            "Wi-Fi Direct".to_string(), // Correct
            "Wi-Fi Mesh".to_string(), // Hard false
            "Wi-Fi Hub".to_string(), // Hard false
            "Wi-Fi Bridge".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following connection types is commonly used for connecting devices like printers and keyboards to a computer without wires?".to_string(),
        vec![
            "Bluetooth".to_string(), // Correct
            "RJ45".to_string(), // Hard false
            "USB-C".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
        ],
        0,
    ));


    it_category.add_question(Question::new(
        "Which routing protocol is used in most modern internet routing?".to_string(), 
        vec![
            "BGP".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "IGRP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does the term 'dynamic routing' refer to?".to_string(), 
        vec![
            "Routes that are automatically updated and maintained by routers".to_string(), // Correct
            "Manually configured routes that don't change".to_string(), // Hard false
            "Routes that are static and need to be manually configured".to_string(), // Hard false
            "Routes that are used only during initial setup".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol is mainly used for routing within a single autonomous system?".to_string(), 
        vec![
            "OSPF".to_string(), // Correct
            "BGP".to_string(), // Hard false
            "RIP".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));
//olkjholdsakjlkj
    //
    it_category.add_question(Question::new(
        "Which of the following devices is responsible for directing traffic within a local network?".to_string(), 
        vec![
            "Router".to_string(), // Correct
            "Switch".to_string(), // Hard false
            "Hub".to_string(), // Hard false
            "Modem".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which routing protocol uses a distance-vector method?".to_string(), 
        vec![
            "RIP".to_string(), // Correct
            "OSPF".to_string(), // Hard false
            "BGP".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What does BGP stand for?".to_string(), 
        vec![
            "Border Gateway Protocol".to_string(), // Correct
            "Binary Gateway Protocol".to_string(), // Hard false
            "Basic Gateway Protocol".to_string(), // Hard false
            "Best Gateway Protocol".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the main role of a default gateway in a network?".to_string(), 
        vec![
            "To route traffic between different networks".to_string(), // Correct
            "To connect devices within a local network".to_string(), // Hard false
            "To assign IP addresses to devices".to_string(), // Hard false
            "To filter incoming and outgoing traffic".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which protocol is used for routing between different autonomous systems?".to_string(), 
        vec![
            "BGP".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which type of routing protocol is OSPF?".to_string(), 
        vec![
            "Link-State".to_string(), // Correct
            "Distance-Vector".to_string(), // Hard false
            "Hybrid".to_string(), // Hard false
            "Path-Vector".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the primary advantage of using OSPF over RIP?".to_string(), 
        vec![
            "Faster convergence time".to_string(), // Correct
            "More secure communication".to_string(), // Hard false
            "Lower resource usage".to_string(), // Hard false
            "Better compatibility with older routers".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is true about the routing protocol RIP?".to_string(), 
        vec![
            "It uses hop count as a metric".to_string(), // Correct
            "It uses bandwidth as a metric".to_string(), // Hard false
            "It supports larger networks than OSPF".to_string(), // Hard false
            "It has faster convergence than OSPF".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which router protocol is considered more efficient for large-scale networks?".to_string(), 
        vec![
            "OSPF".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "BGP".to_string(), // Hard false
            "IGRP".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is a key feature of EIGRP?".to_string(), 
        vec![
            "It uses Diffusing Update Algorithm (DUAL)".to_string(), // Correct
            "It uses link-state information".to_string(), // Hard false
            "It relies on static routes".to_string(), // Hard false
            "It does not support variable-length subnet masking".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "What is the primary difference between static and dynamic routing?".to_string(), 
        vec![
            "Static routing requires manual configuration, dynamic routing adapts automatically".to_string(), // Correct
            "Static routing is more secure than dynamic routing".to_string(), // Hard false
            "Static routing is used only for small networks".to_string(), // Hard false
            "Dynamic routing is not used in large networks".to_string(), // Hard false
        ],
        0,
    ));

    it_category.add_question(Question::new(
        "Which of the following is the main function of a routing table?".to_string(), 
        vec![
            "To store the best paths to destination networks".to_string(), // Correct
            "To filter incoming and outgoing network traffic".to_string(), // Hard false
            "To assign IP addresses to devices".to_string(), // Hard false
            "To store DNS information for domain resolution".to_string(), // Hard false
        ],
        0,
    ));





    // category: abbreviations
    let mut abbreviations_category = Category::new("Abbreviations".to_string());
    abbreviations_category.add_question(Question::new(
        "What does ASAP stand for?".to_string(),
        vec!["As Soon As Possible".to_string(), "As Simple As Possible".to_string()],
        0,
    ));
    abbreviations_category.add_question(Question::new(
        "What does CEO stand for?".to_string(),
        vec!["Chief Executive Officer".to_string(), "Central Executive Officer".to_string()],
        0,
    ));
    abbreviations_category.add_question(Question::new(
        "What does FAQ stand for?".to_string(),
        vec!["Frequently Asked Questions".to_string(), "Frequently Answered Questions".to_string()],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does API stand for?".to_string(),
        vec![
            "Application Programming Interface".to_string(), // Correct
            "Advanced Peripheral Integration".to_string(),    // Hard false
            "Automatic Process Invocation".to_string(),      // Hard false
            "Analytical Programming Interface".to_string(),  // Hard false
        ],
        0, // Correct answer index
    ));

    abbreviations_category.add_question(Question::new(
        "What does ASCII stand for?".to_string(),
        vec![
            "American Standard Code for Information Interchange".to_string(), // Correct
            "Automated System Code for Interface Integration".to_string(),    // Hard false
            "Advanced Standard for Code Interoperation".to_string(),          // Hard false
            "Architectural Syntax Control Information Interface".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does BIOS stand for?".to_string(),
        vec![
            "Basic Input/Output System".to_string(), // Correct
            "Binary Integrated Operating System".to_string(), // Hard false
            "Backend Infrastructure Optimization Software".to_string(), // Hard false
            "Biological Input Output Source".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does BGP stand for?".to_string(),
        vec![
            "Border Gateway Protocol".to_string(), // Correct
            "Binary Gateway Process".to_string(), // Hard false
            "Bridge Group Protocol".to_string(), // Hard false
            "Boundary Graphical Processing".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does CDN stand for?".to_string(),
        vec![
            "Content Delivery Network".to_string(), // Correct
            "Central Data Network".to_string(),     // Hard false
            "Cascading Distribution Node".to_string(), // Hard false
            "Clustered Data Network".to_string(),   // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does CPU stand for?".to_string(),
        vec![
            "Central Processing Unit".to_string(), // Correct
            "Core Peripheral Unit".to_string(),   // Hard false
            "Clustered Processing Utility".to_string(), // Hard false
            "Compute Program Unit".to_string(),   // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does CSS stand for?".to_string(),
        vec![
            "Cascading Style Sheets".to_string(), // Correct
            "Central System Script".to_string(), // Hard false
            "Custom Styling Syntax".to_string(), // Hard false
            "Controlled Style Sheets".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DB stand for?".to_string(),
        vec![
            "Database".to_string(), // Correct
            "Data Block".to_string(), // Hard false
            "Distributed Buffer".to_string(), // Hard false
            "Dynamic Bytecode".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DNS stand for?".to_string(),
        vec![
            "Domain Name System".to_string(), // Correct
            "Distributed Network Service".to_string(), // Hard false
            "Dynamic Node Synchronization".to_string(), // Hard false
            "Domain Networking Suite".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DOS stand for?".to_string(),
        vec![
            "Disk Operating System".to_string(), // Correct
            "Data Operating Solution".to_string(), // Hard false
            "Dynamic Operating System".to_string(), // Hard false
            "Distributed Optimization Software".to_string(), // Hard false
        ],
        0,
    ));
    abbreviations_category.add_question(Question::new(
        "What does ANSI stand for?".to_string(),
        vec![
            "American National Standards Institute".to_string(), // Correct
            "Advanced Network Systems Interface".to_string(),    // Hard false
            "Association of National System Integrators".to_string(), // Hard false
            "Automated Numerical Standards Interface".to_string(), // Hard false
        ],
        0, // Correct answer index
    ));

    abbreviations_category.add_question(Question::new(
        "What does ARP stand for?".to_string(),
        vec![
            "Address Resolution Protocol".to_string(), // Correct
            "Application Routing Protocol".to_string(), // Hard false
            "Advanced Redirection Process".to_string(), // Hard false
            "Automated Request Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ATM stand for?".to_string(),
        vec![
            "Asynchronous Transfer Mode".to_string(), // Correct
            "Automated Teller Machine".to_string(), // Trick answer
            "Active Transmission Matrix".to_string(), // Hard false
            "Advanced Transfer Management".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does BFS stand for?".to_string(),
        vec![
            "Breadth-First Search".to_string(), // Correct
            "Binary File System".to_string(),  // Hard false
            "Backup File Synchronization".to_string(), // Hard false
            "Basic File Search".to_string(),   // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does BT stand for?".to_string(),
        vec![
            "Bluetooth".to_string(), // Correct
            "Binary Transfer".to_string(), // Hard false
            "Basic Telecommunication".to_string(), // Hard false
            "Broadband Transmission".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does CI/CD stand for?".to_string(),
        vec![
            "Continuous Integration/Continuous Deployment".to_string(), // Correct
            "Code Integration/Code Development".to_string(), // Hard false
            "Continuous Improvement/Controlled Delivery".to_string(), // Hard false
            "Critical Infrastructure/Cloud Deployment".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does CRC stand for?".to_string(),
        vec![
            "Cyclic Redundancy Check".to_string(), // Correct
            "Critical Redundancy Check".to_string(), // Hard false
            "Controlled Routing Cipher".to_string(), // Hard false
            "Code Redundancy Calculation".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DHCP stand for?".to_string(),
        vec![
            "Dynamic Host Configuration Protocol".to_string(), // Correct
            "Distributed Host Communication Protocol".to_string(), // Hard false
            "Data Handling Configuration Protocol".to_string(), // Hard false
            "Dynamic Hypertext Control Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DNSSEC stand for?".to_string(),
        vec![
            "DNS Security Extensions".to_string(), // Correct
            "Distributed Node Security System".to_string(), // Hard false
            "Dynamic Name Service Encryption Control".to_string(), // Hard false
            "Domain Name System Encrypted Channel".to_string(), // Hard false
        ],
        0,
    ));
    abbreviations_category.add_question(Question::new(
        "What does DR stand for?".to_string(),
        vec![
            "Disaster Recovery".to_string(), // Correct
            "Data Redundancy".to_string(),  // Hard false
            "Distributed Resources".to_string(), // Hard false
            "Dynamic Replication".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does FAT stand for?".to_string(),
        vec![
            "File Allocation Table".to_string(), // Correct
            "Fast Access Table".to_string(), // Hard false
            "File Attribute Tracker".to_string(), // Hard false
            "Flexible Archive Tool".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does GPU stand for?".to_string(),
        vec![
            "Graphics Processing Unit".to_string(), // Correct
            "General Purpose Unit".to_string(), // Hard false
            "Graphics Protocol Utility".to_string(), // Hard false
            "Global Performance Unit".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does HTML stand for?".to_string(),
        vec![
            "HyperText Markup Language".to_string(), // Correct
            "High-Tech Metadata Layer".to_string(), // Hard false
            "HyperText Module Language".to_string(), // Hard false
            "Hierarchical Template Modeling Language".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does IPsec stand for?".to_string(),
        vec![
            "Internet Protocol Security".to_string(), // Correct
            "IP Secure Protocol".to_string(), // Hard false
            "Internet Privacy Security".to_string(), // Hard false
            "Integrated Protocol Security".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does JVM stand for?".to_string(),
        vec![
            "Java Virtual Machine".to_string(), // Correct
            "Just-in-time Virtual Manager".to_string(), // Hard false
            "Java Version Manager".to_string(), // Hard false
            "Java Variable Management".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does NFS stand for?".to_string(),
        vec![
            "Network File System".to_string(), // Correct
            "Node File Storage".to_string(), // Hard false
            "Network File Synchronization".to_string(), // Hard false
            "Net Folder Sharing".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does OOP stand for?".to_string(),
        vec![
            "Object-Oriented Programming".to_string(), // Correct
            "Optimal Operations Protocol".to_string(), // Hard false
            "Object Orchestration Process".to_string(), // Hard false
            "Operational Output Programming".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RAID stand for?".to_string(),
        vec![
            "Redundant Array of Independent Disks".to_string(), // Correct
            "Rapid Access Internal Disks".to_string(), // Hard false
            "Remote Array of Integrated Devices".to_string(), // Hard false
            "Redundant Access Input Data".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SMTP stand for?".to_string(),
        vec![
            "Simple Mail Transfer Protocol".to_string(), // Correct
            "Secure Mail Transmission Protocol".to_string(), // Hard false
            "Standard Message Transport Protocol".to_string(), // Hard false
            "Simple Messaging Transport Procedure".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does WLAN stand for?".to_string(),
        vec![
            "Wireless Local Area Network".to_string(), // Correct
            "Wide Local Access Network".to_string(),  // Hard false
            "Wireless Linked Access Node".to_string(), // Hard false
            "Wide Loop Area Network".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does WWW stand for?".to_string(),
        vec![
            "World Wide Web".to_string(), // Correct
            "Wide World Web".to_string(), // Hard false
            "Web Without Wires".to_string(), // Hard false
            "Wide Wireless Web".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does XML stand for?".to_string(),
        vec![
            "eXtensible Markup Language".to_string(), // Correct
            "eXtended Metadata Layer".to_string(), // Hard false
            "eXclusive Model Language".to_string(), // Hard false
            "eXchange Markup Layout".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does YAML stand for?".to_string(),
        vec![
            "YAML Ain't Markup Language".to_string(), // Correct
            "Yet Another Markup Language".to_string(), // Hard false
            "Your Accessible Markup Layout".to_string(), // Hard false
            "You Always Make Layouts".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ZIP stand for?".to_string(),
        vec![
            "Zone Improvement Plan".to_string(), // Correct
            "Zero Integration Protocol".to_string(), // Hard false
            "Zipped Internet Protocol".to_string(), // Hard false
            "Zone Interchange Package".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ACL stand for?".to_string(),
        vec![
            "Access Control List".to_string(), // Correct
            "Advanced Configuration Layer".to_string(), // Hard false
            "Application Connection Log".to_string(), // Hard false
            "Authorized Control Ledger".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does AES stand for?".to_string(),
        vec![
            "Advanced Encryption Standard".to_string(), // Correct
            "Automated Encryption System".to_string(), // Hard false
            "Advanced Encoding Schema".to_string(), // Hard false
            "Authenticated Encryption Service".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does AI stand for?".to_string(),
        vec![
            "Artificial Intelligence".to_string(), // Correct
            "Automated Integration".to_string(), // Hard false
            "Augmented Interpretation".to_string(), // Hard false
            "Advanced Interaction".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does AJAX stand for?".to_string(),
        vec![
            "Asynchronous JavaScript and XML".to_string(), // Correct
            "Advanced JavaScript and XML".to_string(), // Hard false
            "Automated JavaScript Application".to_string(), // Hard false
            "Assembled JavaScript and XHTML".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does UDP stand for?".to_string(),
        vec![
            "User Datagram Protocol".to_string(), // Correct
            "Universal Data Packet".to_string(), // Hard false
            "User Data Protocol".to_string(), // Hard false
            "Unified Datagram Processing".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does URL stand for?".to_string(),
        vec![
            "Uniform Resource Locator".to_string(), // Correct
            "Universal Reference Link".to_string(), // Hard false
            "Uniform Retrieval Location".to_string(), // Hard false
            "Unique Resource Link".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does USB stand for?".to_string(),
        vec![
            "Universal Serial Bus".to_string(), // Correct
            "Unified System Bus".to_string(), // Hard false
            "Universal Storage Bridge".to_string(), // Hard false
            "Universal Signal Bus".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does UUID stand for?".to_string(),
        vec![
            "Universally Unique Identifier".to_string(), // Correct
            "Universal Unit Identifier".to_string(), // Hard false
            "Unique Universal ID".to_string(), // Hard false
            "Unified Unique Index".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does VLAN stand for?".to_string(),
        vec![
            "Virtual Local Area Network".to_string(), // Correct
            "Virtual Linked Access Node".to_string(), // Hard false
            "Virtual Loopback Area Network".to_string(), // Hard false
            "Virtual Local Access Network".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does VM stand for?".to_string(),
        vec![
            "Virtual Machine".to_string(), // Correct
            "Virtual Module".to_string(), // Hard false
            "Verified Machine".to_string(), // Hard false
            "Virtual Memory".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does VPN stand for?".to_string(),
        vec![
            "Virtual Private Network".to_string(), // Correct
            "Virtual Public Node".to_string(), // Hard false
            "Verified Private Network".to_string(), // Hard false
            "Virtual Proxy Network".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does VRAM stand for?".to_string(),
        vec![
            "Video Random-Access Memory".to_string(), // Correct
            "Virtual Random-Access Memory".to_string(), // Hard false
            "Video Read-Access Memory".to_string(), // Hard false
            "Verified Random-Access Module".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does WAN stand for?".to_string(),
        vec![
            "Wide Area Network".to_string(), // Correct
            "Wireless Access Node".to_string(), // Hard false
            "Wide Access Network".to_string(), // Hard false
            "Wireless Area Network".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does WiFi stand for?".to_string(),
        vec![
            "Wireless Fidelity".to_string(), // Correct
            "Wide Fidelity".to_string(), // Hard false
            "Wireless Frequency".to_string(), // Hard false
            "Wide Frequency Interface".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SDK stand for?".to_string(),
        vec![
            "Software Development Kit".to_string(), // Correct
            "System Design Kit".to_string(),       // Hard false
            "Standard Deployment Kit".to_string(), // Hard false
            "Software Deployment Kernel".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SMTP stand for?".to_string(),
        vec![
            "Simple Mail Transfer Protocol".to_string(), // Correct
            "Secure Mail Transmission Protocol".to_string(), // Hard false
            "System Mail Transfer Protocol".to_string(), // Hard false
            "Simple Messaging Transport Process".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SNMP stand for?".to_string(),
        vec![
            "Simple Network Management Protocol".to_string(), // Correct
            "Secure Network Messaging Protocol".to_string(), // Hard false
            "System Network Management Process".to_string(), // Hard false
            "Standard Network Management Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SQL stand for?".to_string(),
        vec![
            "Structured Query Language".to_string(), // Correct
            "Simple Query Language".to_string(), // Hard false
            "Secure Query Language".to_string(), // Hard false
            "Standard Query Logic".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SSH stand for?".to_string(),
        vec![
            "Secure Shell".to_string(), // Correct
            "Simple System Host".to_string(), // Hard false
            "Secure Socket Host".to_string(), // Hard false
            "System Shell Handler".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SSL stand for?".to_string(),
        vec![
            "Secure Sockets Layer".to_string(), // Correct
            "Secure Socket Link".to_string(), // Hard false
            "Standard Security Layer".to_string(), // Hard false
            "Secure Server Layer".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SSD stand for?".to_string(),
        vec![
            "Solid-State Drive".to_string(), // Correct
            "Standard Storage Device".to_string(), // Hard false
            "Solid Storage Disk".to_string(), // Hard false
            "Static Storage Device".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does TCP stand for?".to_string(),
        vec![
            "Transmission Control Protocol".to_string(), // Correct
            "Transfer Communication Protocol".to_string(), // Hard false
            "Transport Control Process".to_string(), // Hard false
            "Transmission Communication Path".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does TLS stand for?".to_string(),
        vec![
            "Transport Layer Security".to_string(), // Correct
            "Transmission Link Security".to_string(), // Hard false
            "Transport Link System".to_string(), // Hard false
            "Trusted Layer Security".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does TLD stand for?".to_string(),
        vec![
            "Top-Level Domain".to_string(), // Correct
            "Transport Layer Domain".to_string(), // Hard false
            "Trusted Link Domain".to_string(), // Hard false
            "Transmission Level Directory".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PWM stand for?".to_string(),
        vec![
            "Pulse Width Modulation".to_string(), // Correct
            "Power Wave Modulation".to_string(), // Hard false
            "Pulse Wave Modulator".to_string(), // Hard false
            "Power Width Manager".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RAM stand for?".to_string(),
        vec![
            "Random-Access Memory".to_string(), // Correct
            "Read-Only Memory".to_string(), // Hard false
            "Rapid Access Module".to_string(), // Hard false
            "Random Allocation Memory".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RDP stand for?".to_string(),
        vec![
            "Remote Desktop Protocol".to_string(), // Correct
            "Resource Data Protocol".to_string(), // Hard false
            "Remote Data Process".to_string(), // Hard false
            "Remote Device Path".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does REST stand for?".to_string(),
        vec![
            "Representational State Transfer".to_string(), // Correct
            "Remote Synchronous Transfer".to_string(), // Hard false
            "Resource State Transport".to_string(), // Hard false
            "Representational Service Transfer".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ROM stand for?".to_string(),
        vec![
            "Read-Only Memory".to_string(), // Correct
            "Random-Only Memory".to_string(), // Hard false
            "Resource Optimization Module".to_string(), // Hard false
            "Read-On Module".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RPC stand for?".to_string(),
        vec![
            "Remote Procedure Call".to_string(), // Correct
            "Resource Process Communication".to_string(), // Hard false
            "Remote Process Command".to_string(), // Hard false
            "Rapid Process Communication".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RSS stand for?".to_string(),
        vec![
            "Really Simple Syndication".to_string(), // Correct
            "Resource Syndication Service".to_string(), // Hard false
            "Rapid Subscription Service".to_string(), // Hard false
            "Remote Syndication System".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does RTOS stand for?".to_string(),
        vec![
            "Real-Time Operating System".to_string(), // Correct
            "Remote Task Operating System".to_string(), // Hard false
            "Real-Time Optimization System".to_string(), // Hard false
            "Rapid Task Optimization Software".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SaaS stand for?".to_string(),
        vec![
            "Software as a Service".to_string(), // Correct
            "System as a Service".to_string(), // Hard false
            "Storage as a Service".to_string(), // Hard false
            "Software Application Subscription".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SAN stand for?".to_string(),
        vec![
            "Storage Area Network".to_string(), // Correct
            "System Area Network".to_string(), // Hard false
            "Secure Access Network".to_string(), // Hard false
            "Shared Application Node".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ML stand for?".to_string(),
        vec![
            "Machine Learning".to_string(), // Correct
            "Markup Language".to_string(), // Hard false
            "Matrix Logic".to_string(), // Hard false
            "Modular Learning".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does NAT stand for?".to_string(),
        vec![
            "Network Address Translation".to_string(), // Correct
            "Node Allocation Table".to_string(), // Hard false
            "Network Allocation Tracker".to_string(), // Hard false
            "Node Address Translator".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does NFC stand for?".to_string(),
        vec![
            "Near Field Communication".to_string(), // Correct
            "Network File Communication".to_string(), // Hard false
            "Near Frequency Connection".to_string(), // Hard false
            "Network Frequency Controller".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does OS stand for?".to_string(),
        vec![
            "Operating System".to_string(), // Correct
            "Operational Software".to_string(), // Hard false
            "Online Service".to_string(), // Hard false
            "Open Source".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PaaS stand for?".to_string(),
        vec![
            "Platform as a Service".to_string(), // Correct
            "Product as a Service".to_string(), // Hard false
            "Protocol as a Service".to_string(), // Hard false
            "Process as a Service".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PCIe stand for?".to_string(),
        vec![
            "Peripheral Component Interconnect Express".to_string(), // Correct
            "Peripheral Connection Interface Express".to_string(), // Hard false
            "Protocol Component Integration Express".to_string(), // Hard false
            "Peripheral Control Integration Express".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PDF stand for?".to_string(),
        vec![
            "Portable Document Format".to_string(), // Correct
            "Personal Data File".to_string(), // Hard false
            "Public Data Framework".to_string(), // Hard false
            "Protocol Document File".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PHP stand for?".to_string(),
        vec![
            "PHP: Hypertext Preprocessor".to_string(), // Correct
            "Programming Hypertext Protocol".to_string(), // Hard false
            "Precompiled Hypertext Processor".to_string(), // Hard false
            "Protocol Hypertext Parser".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does POP3 stand for?".to_string(),
        vec![
            "Post Office Protocol 3".to_string(), // Correct
            "Protocol of Posts 3".to_string(), // Hard false
            "Post Operational Protocol 3".to_string(), // Hard false
            "Processing of Posts 3".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does PSU stand for?".to_string(),
        vec![
            "Power Supply Unit".to_string(), // Correct
            "Power System Unit".to_string(), // Hard false
            "Primary Supply Unit".to_string(), // Hard false
            "Peripheral Supply Unit".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does DRAM stand for?".to_string(),
        vec![
            "Dynamic Random-Access Memory".to_string(), // Correct
            "Dynamic Read-Access Memory".to_string(), // Hard false
            "Direct Random-Access Memory".to_string(), // Hard false
            "Dual Random-Access Memory".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does ERP stand for?".to_string(),
        vec![
            "Enterprise Resource Planning".to_string(), // Correct
            "Enterprise Resource Processor".to_string(), // Hard false
            "External Resource Planning".to_string(), // Hard false
            "Enterprise Retrieval Process".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does FTP stand for?".to_string(),
        vec![
            "File Transfer Protocol".to_string(), // Correct
            "File Transport Protocol".to_string(), // Hard false
            "Fast Transfer Protocol".to_string(), // Hard false
            "File Technical Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does GUI stand for?".to_string(),
        vec![
            "Graphical User Interface".to_string(), // Correct
            "Graphical Universal Interface".to_string(), // Hard false
            "Global User Interface".to_string(), // Hard false
            "General User Input".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does HTTP stand for?".to_string(),
        vec![
            "HyperText Transfer Protocol".to_string(), // Correct
            "Hyper Terminal Transfer Protocol".to_string(), // Hard false
            "HyperText Transmission Protocol".to_string(), // Hard false
            "Hyper Transfer Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does HTTPS stand for?".to_string(),
        vec![
            "HyperText Transfer Protocol Secure".to_string(), // Correct
            "HyperText Transmission Protocol Secure".to_string(), // Hard false
            "Hyper Terminal Transfer Protocol Secure".to_string(), // Hard false
            "Hyper Transfer Protocol with Security".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does IDE stand for?".to_string(),
        vec![
            "Integrated Development Environment".to_string(), // Correct
            "Interactive Development Environment".to_string(), // Hard false
            "Integrated Data Environment".to_string(), // Hard false
            "International Development Environment".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does IP stand for?".to_string(),
        vec![
            "Internet Protocol".to_string(), // Correct
            "Internet Processor".to_string(), // Hard false
            "Internal Protocol".to_string(), // Hard false
            "International Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does IPv4 stand for?".to_string(),
        vec![
            "Internet Protocol Version 4".to_string(), // Correct
            "Internet Processor Version 4".to_string(), // Hard false
            "Internal Protocol Version 4".to_string(), // Hard false
            "Internet Protocol Version Four".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does IPv6 stand for?".to_string(),
        vec![
            "Internet Protocol Version 6".to_string(), // Correct
            "Internet Processor Version 6".to_string(), // Hard false
            "Internal Protocol Version 6".to_string(), // Hard false
            "Internet Protocol Version Six".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does SOAP stand for?".to_string(),
        vec![
            "Simple Object Access Protocol".to_string(), // Correct
            "Simple Online Access Protocol".to_string(), // Hard false
            "Structured Object Access Protocol".to_string(), // Hard false
            "Secure Object Access Protocol".to_string(), // Hard false
        ],
        0,
    ));

    abbreviations_category.add_question(Question::new(
        "What does VNC stand for?".to_string(),
        vec![
            "Virtual Network Computing".to_string(), // Correct
            "Virtual Node Computing".to_string(), // Hard false
            "Virtual Network Communication".to_string(), // Hard false
            "Virtual Network Controller".to_string(), // Hard false
        ],
        0,
    ));



    // category IPV4
    let mut ipv4_classes_category = Category::new("IPV4".to_string());
    
    // Create 10 random questions
    for _ in 0..10 {
        ipv4_classes_category.add_question(generate_ipv4_question());
    }



    // add categories to quiz
    quiz.add_category(math_category);
    quiz.add_category(it_category);
    quiz.add_category(abbreviations_category);
    quiz.add_category(ipv4_classes_category);

    quiz
}

