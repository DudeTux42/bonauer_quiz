use crate::models::Category;
use crate::models::Question;

pub fn add_it_questions(category: &mut Category) {
    category.add_question(Question::new(
        "What is the primary language used for web development?".to_string(),
        vec!["Java".to_string(), "JavaScript".to_string(), "C++".to_string()],
        1,
    ));

    category.add_question(Question::new(
        "Who is known as the father of Free Software Foundation?".to_string(),
        vec![
            "Richard Mathew Stallman".to_string(), // Correct
            "Linus Torvalds".to_string(), // Hard false
            "Ken Thompson".to_string(), // Hard false
            "Dennis Ritchie".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "In which year was the ‘C’ programming language developed?".to_string(),
        vec![
            "1972".to_string(), // Correct
            "1965".to_string(), // Hard false
            "1980".to_string(), // Hard false
            "1990".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is known as the father of Internet?".to_string(),
        vec![
            "Vinton Cerf".to_string(), // Correct
            "Tim Berners-Lee".to_string(), // Hard false
            "Marc Andreessen".to_string(), // Hard false
            "Robert Kahn".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who used the binary system of numeration for the first time?".to_string(),
        vec![
            "Thomas Harriot".to_string(), // Correct
            "John von Neumann".to_string(), // Hard false
            "Isaac Newton".to_string(), // Hard false
            "Charles Babbage".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is the first computer programmer?".to_string(),
        vec![
            "Ada Lovelace".to_string(), // Correct
            "Grace Hopper".to_string(), // Hard false
            "John von Neumann".to_string(), // Hard false
            "Alan Turing".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "In which year was computer graphics originated?".to_string(),
        vec![
            "1940".to_string(), // Correct
            "1930".to_string(), // Hard false
            "1950".to_string(), // Hard false
            "1960".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is the inventor of Supercomputer?".to_string(),
        vec![
            "Seymour Cray".to_string(), // Correct
            "Bill Gates".to_string(), // Hard false
            "Mark Zuckerberg".to_string(), // Hard false
            "Steve Jobs".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name of World’s first digital Computer?".to_string(),
        vec![
            "Mark 1".to_string(), // Correct
            "ENIAC".to_string(), // Hard false
            "Colossus".to_string(), // Hard false
            "Atanasoff-Berry Computer".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name of India’s first indigenous supercomputer developed by CDAC?".to_string(),
        vec![
            "Param".to_string(), // Correct
            "Brahmos".to_string(), // Hard false
            "Sahasra".to_string(), // Hard false
            "Vishal".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which was the first Pocket Computer?".to_string(),
        vec![
            "SHARP PC1211".to_string(), // Correct
            "Casio fx-7000G".to_string(), // Hard false
            "HP-35".to_string(), // Hard false
            "TRS-80".to_string(), // Hard false
        ],
        0,
    ));
    

    category.add_question(Question::new(
        "What is the name of the tablet introduced by Amazon?".to_string(),
        vec![
            "Kindle Fire".to_string(), // Correct
            "Nook Tablet".to_string(), // Hard false
            "Surface Pro".to_string(), // Hard false
            "iPad Mini".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who invented the Computer Mouse?".to_string(),
        vec![
            "Douglas Engelbart".to_string(), // Correct
            "Alan Turing".to_string(), // Hard false
            "John von Neumann".to_string(), // Hard false
            "Bill Gates".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Speak to Tweet is a service given by which giant?".to_string(),
        vec![
            "Google".to_string(), // Correct
            "Twitter".to_string(), // Hard false
            "Microsoft".to_string(), // Hard false
            "Facebook".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Expand HDMI".to_string(),
        vec![
            "High Definition Multimedia Interface".to_string(), // Correct
            "High Density Media Interface".to_string(), // Hard false
            "High Data Multimedia Integration".to_string(), // Hard false
            "High Definition Media Interconnect".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was developed by PYRA Labs and later sold to Google?".to_string(),
        vec![
            "Blogger".to_string(), // Correct
            "YouTube".to_string(), // Hard false
            "Drive".to_string(), // Hard false
            "Chrome".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is known as the father of the World Wide Web?".to_string(),
        vec![
            "Tim Berners-Lee".to_string(), // Correct
            "Vinton Cerf".to_string(), // Hard false
            "Robert Kahn".to_string(), // Hard false
            "Marc Andreessen".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which method is used to connect to a remote computer?".to_string(),
        vec![
            "VPN".to_string(),          // Plausible but incorrect
            "Telnet".to_string(),       // Outdated and insecure
            "File Transfer Protocol".to_string(), // Incorrect usage
            "Remote Desktop Protocol".to_string(), // Correct
        ],
        3,
    ));

    category.add_question(Question::new(
        "What do you call a single point on a computer screen?".to_string(),
        vec![
            "Pixel".to_string(), // Correct
            "Bit".to_string(), // Hard false
            "Dot".to_string(), // Hard false
            "Node".to_string(), // Hard false
        ],
        0,
    ));
    
    
    category.add_question(Question::new(
        "The CPU chip used in a computer is made of which element?".to_string(),
        vec![
            "Silica".to_string(), // Correct
            "Copper".to_string(), // Hard false
            "Graphite".to_string(), // Hard false
            "Plastic".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which computer was designed to be as compact as possible?".to_string(),
        vec![
            "Micro Computer".to_string(), // Correct
            "Super Computer".to_string(), // Hard false
            "Mainframe".to_string(), // Hard false
            "Workstation".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name of the device which produces hard copy graphics?".to_string(),
        vec![
            "Plotter".to_string(), // Correct
            "Printer".to_string(), // Hard false
            "Scanner".to_string(), // Hard false
            "Copier".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "When is World Computer Literacy Day celebrated?".to_string(),
        vec![
            "December 2".to_string(), // Correct
            "January 15".to_string(), // Hard false
            "June 10".to_string(), // Hard false
            "September 25".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "The processing power of a CPU is measured in?".to_string(),
        vec![
            "MIPS".to_string(), // Correct
            "GHz".to_string(), // Hard false
            "Bits".to_string(), // Hard false
            "Cores".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Windows 10 was released on which day?".to_string(),
        vec![
            "July 29".to_string(), // Correct
            "August 15".to_string(), // Hard false
            "October 5".to_string(), // Hard false
            "September 1".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which command is used to view the sub-directory structure of a drive?".to_string(),
        vec![
            "TREE".to_string(), // Correct
            "DIR".to_string(), // Hard false
            "LS".to_string(), // Hard false
            "FIND".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Round Robin scheduling is the preemptive version of?".to_string(),
        vec![
            "FIFO".to_string(), // Correct
            "LIFO".to_string(), // Hard false
            "SJF".to_string(), // Hard false
            "Priority".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "When did IBM release its first version of DOS OS 1.0?".to_string(),
        vec![
            "1981".to_string(), // Correct
            "1980".to_string(), // Hard false
            "1982".to_string(), // Hard false
            "1979".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "The Start button was introduced in which operating system?".to_string(),
        vec![
            "Windows 95".to_string(), // Correct
            "Windows 98".to_string(), // Hard false
            "Windows XP".to_string(), // Hard false
            "Windows ME".to_string(), // Hard false
        ],
        0,
    ));



    category.add_question(Question::new(
        "Which system software does the job of merging the records from two files to one?".to_string(),
        vec![
            "Database Mangement System (DBMS)".to_string(), // Correct
            "Integrated Development Environment".to_string(), // Hard false
            "File Manager".to_string(), // Hard false
            "Compiler".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "While running DOS on a computer, which command is used to duplicate the entire diskette?".to_string(),
        vec![
            "DISKCOPY".to_string(), // Correct
            "COPY".to_string(), // Hard false
            "XCOPY".to_string(), // Hard false
            "FORMAT".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name given to the organized collection of software that controls the entire operation of a computer?".to_string(),
        vec![
            "Operating System".to_string(), // Correct
            "Application Software".to_string(), // Hard false
            "Device Driver".to_string(), // Hard false
            "BIOS".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name of the OS for the laptop MACLITE?".to_string(),
        vec![
            "OZ".to_string(), // Correct
            "MACOS".to_string(), // Hard false
            "LiteOS".to_string(), // Hard false
            "Catalina".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the memory range from 1k-640k called?".to_string(),
        vec![
            "Conventional memory".to_string(), // Correct
            "Virtual memory".to_string(), // Hard false
            "Extended memory".to_string(), // Hard false
            "Cache memory".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "IBM released its first PC in 1981, what was the name of the OS that was popular at that time?".to_string(),
        vec![
            "CP/M".to_string(), // Correct
            "DOS".to_string(), // Hard false
            "UNIX".to_string(), // Hard false
            "Windows 1.0".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "How long is an IPv6 Address?".to_string(),
        vec![
            "128 bit (16 byte)".to_string(), // Correct
            "64 bit (8 byte)".to_string(), // Hard false
            "32 bit (4 byte)".to_string(), // Hard false
            "256 bit (32 byte)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol does DHCP use at the transport layer?".to_string(),
        vec![
            "UDP".to_string(), // Correct
            "TCP".to_string(), // Hard false
            "ICMP".to_string(), // Hard false
            "HTTP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol is used to send a destination network unknown message back to the originating host?".to_string(),
        vec![
            "ICMP".to_string(), // Correct
            "ARP".to_string(), // Hard false
            "IP".to_string(), // Hard false
            "SMTP".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "Which WLAN IEEE Specification allows up to 54 Mbps at 2.4 GHz?".to_string(),
        vec![
            "IEEE 802.11G".to_string(), // Correct
            "IEEE 802.11B".to_string(), // Hard false
            "IEEE 802.11N".to_string(), // Hard false
            "IEEE 802.11A".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol does PPP use to identify the network layer protocol?".to_string(),
        vec![
            "NCP".to_string(), // Correct
            "LCP".to_string(), // Hard false
            "ICMP".to_string(), // Hard false
            "DHCP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "HBA stands for?".to_string(),
        vec![
            "Host Bus Adapter".to_string(), // Correct
            "Hyper Bus Array".to_string(), // Hard false
            "Hardware Base Adapter".to_string(), // Hard false
            "High Bandwidth Access".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the second generation of the Web called?".to_string(),
        vec![
            "Web 2.0".to_string(), // Correct
            "Web 1.0".to_string(), // Hard false
            "Semantic Web".to_string(), // Hard false
            "Dynamic Web".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "If you have a CISCO Mesh network, what protocol allows multiple APs to connect with many redundant connections between nodes?".to_string(),
        vec![
            "AWPP".to_string(), // Correct
            "BGP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
            "RIP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol is used to identify the hardware address of a local device?".to_string(),
        vec![
            "Address Resolution Protocol".to_string(), // Correct
            "Dynamic Host Configuration Protocol".to_string(), // Hard false
            "Internet Control Message Protocol".to_string(), // Hard false
            "Transmission Control Protocol".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "PAT Address Translation is also termed as what?".to_string(),
        vec![
            "NAT Overload".to_string(), // Correct
            "Static NAT".to_string(), // Hard false
            "Dynamic NAT".to_string(), // Hard false
            "Proxy ARP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "When was the term Social Networking first used?".to_string(),
        vec![
            "1954".to_string(), // Correct
            "1994".to_string(), // Hard false
            "2004".to_string(), // Hard false
            "1974".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who founded 'MySpace'?".to_string(),
        vec![
            "Tom Anderson".to_string(), // Correct
            "Mark Zuckerberg".to_string(), // Hard false
            "Jack Dorsey".to_string(), // Hard false
            "Evan Spiegel".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What type of audience are primarily in a social network?".to_string(),
        vec![
            "Joiners".to_string(), // Correct
            "Creators".to_string(), // Hard false
            "Critics".to_string(), // Hard false
            "Spectators".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Twitter is an example of what service?".to_string(),
        vec![
            "Micro Blogging".to_string(), // Correct
            "Social Networking".to_string(), // Hard false
            "News Aggregation".to_string(), // Hard false
            "Instant Messaging".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the character limit for a tweet in Twitter?".to_string(),
        vec![
            "280".to_string(), // Correct 
            "140".to_string(), // Hard false
            "160".to_string(), // Hard false
            "200".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was the largest social network prior to Facebook?".to_string(),
        vec![
            "MySpace".to_string(), // Correct
            "Orkut".to_string(), // Hard false
            "LinkedIn".to_string(), // Hard false
            "Hi5".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "When did social networking first become popular online?".to_string(),
        vec![
            "2003".to_string(), // Correct
            "2005".to_string(), // Hard false
            "2000".to_string(), // Hard false
            "1999".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is YouTube’s slogan?".to_string(),
        vec![
            "Broadcast Yourself".to_string(), // Correct
            "Stream the World".to_string(), // Hard false
            "Watch and Share".to_string(), // Hard false
            "Create and Connect".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "P2P, B2B, and B2C are part of?".to_string(),
        vec![
            "Share Economy".to_string(), // Correct
            "Cryptocurrency".to_string(), // Hard false
            "Online Marketplaces".to_string(), // Hard false
            "Blockchain Technology".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Winchester drive is also called as what?".to_string(),
        vec![
            "Hard disk drive".to_string(), // Correct
            "Solid-State Drive".to_string(), // Hard false
            "Floppy Drive".to_string(), // Hard false
            "Tape Drive".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "What kind of connectors are used to connect a PC power supply to a hardware?".to_string(),
        vec![
            "Molex".to_string(), // Correct
            "SATA".to_string(), // Hard false
            "PCIe".to_string(), // Hard false
            "USB".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the term ‘Wave Table Synthesis’ related to?".to_string(),
        vec![
            "Sound".to_string(), // Correct
            "Graphics".to_string(), // Hard false
            "Networking".to_string(), // Hard false
            "Storage".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What type of memory is Pendrive?".to_string(),
        vec![
            "Flash Memory".to_string(), // Correct
            "RAM".to_string(), // Hard false
            "ROM".to_string(), // Hard false
            "Cache".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which IRQ does the hard disk drive use?".to_string(),
        vec![
            "14".to_string(), // Correct
            "10".to_string(), // Hard false
            "5".to_string(), // Hard false
            "7".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who invented Optical Disc Media?".to_string(),
        vec![
            "James Russel".to_string(), // Correct
            "Philips".to_string(), // Hard false
            "Sony".to_string(), // Hard false
            "George Eastman".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is ‘Arrandale’ code name for?".to_string(),
        vec![
            "Intel Processor".to_string(), // Correct
            "AMD Graphics Card".to_string(), // Hard false
            "NVIDIA Chipset".to_string(), // Hard false
            "Apple M1 Chip".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What hardware was used by the initial generation of computers?".to_string(),
        vec![
            "Valves".to_string(), // Correct
            "Transistors".to_string(), // Hard false
            "Microchips".to_string(), // Hard false
            "Integrated Circuits".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which was the first computer made available for commercial use?".to_string(),
        vec![
            "UNIVAC".to_string(), // Correct
            "ENIAC".to_string(), // Hard false
            "IBM PC".to_string(), // Hard false
            "Altair 8800".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Name the first mechanical computer designed by Charles Babbage called?".to_string(),
        vec![
            "Analytical Engine".to_string(), // Correct
            "Difference Engine".to_string(), // Hard false
            "Turing Machine".to_string(), // Hard false
            "Zuse Z3".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "The concentric circles on the platter of a hard disk are known as?".to_string(),
        vec![
            "Tracks".to_string(), // Correct
            "Sectors".to_string(), // Hard false
            "Clusters".to_string(), // Hard false
            "Cylinders".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "IRQ6 is commonly assigned to?".to_string(),
        vec![
            "Floppy Drive Controller".to_string(), // Correct
            "Hard Disk Controller".to_string(), // Hard false
            "Sound Card".to_string(), // Hard false
            "Network Adapter".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which component in a PC regulates the color depth and screen resolution of a monitor?".to_string(),
        vec![
            "VRAM".to_string(), // Correct
            "GPU".to_string(), // Hard false
            "Monitor".to_string(), // Hard false
            "RAM".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "A Computer programming language for simulating models of business activity is?".to_string(),
        vec![
            "GPSS".to_string(), // Correct
            "COBOL".to_string(), // Hard false
            "BASIC".to_string(), // Hard false
            "Simula".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "The words that are set aside by the programming language for its own use are called as what?".to_string(),
        vec![
            "Control Structures".to_string(), // Correct
            "Variables".to_string(), // Hard false
            "Functions".to_string(), // Hard false
            "Classes".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which programming language is used for scientific calculations?".to_string(),
        vec![
            "FORTRAN".to_string(), // Correct
            "MATLAB".to_string(), // Hard false
            "C".to_string(), // Hard false
            "Python".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which JavaScript framework is most commonly used for modern web development?".to_string(),
        vec![
            "React".to_string(),    // Correct
            "Svelte".to_string(),   // Modern, but less common
            "Ember.js".to_string(), // Still used, but outdated compared to React
            "Backbone.js".to_string(), // Very outdated, not commonly used anymore
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which language is used for the development of various games?".to_string(),
        vec![
            "C++".to_string(), // Correct
            "Python".to_string(), // Hard false
            "JavaScript".to_string(), // Hard false
            "Rust".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which language was devised by Dr. Seymour Papert?".to_string(),
        vec![
            "LOGO".to_string(), // Correct
            "Pascal".to_string(), // Hard false
            "LISP".to_string(), // Hard false
            "COBOL".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which computer language is used for Artificial Intelligence?".to_string(),
        vec![
            "Prolog".to_string(), // Correct
            "Python".to_string(), // Hard false
            "LISP".to_string(), // Hard false
            "Java".to_string(), // Hard false
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who is the creator of the Pascal language?".to_string(),
        vec![
            "Niclaus Wirth".to_string(), // Correct
            "Dennis Ritchie".to_string(), // Hard false
            "Bjarne Stroustrup".to_string(), // Hard false
            "James Gosling".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "Which systems programming environment is commonly used for microcontrollers like ESP32?".to_string(),
        vec![
            "C++".to_string(),   // Correct
            "Python".to_string(), // Plausible, used for simpler applications
            "JavaScript".to_string(), // Used with frameworks like NodeMCU for microcontrollers
            "Rust".to_string(),  // Increasingly popular, but less common than C++ for embedded systems
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which programming language is known for supporting structured programming with modern features?".to_string(),
        vec![
            "Rust".to_string(),      // Correct
            "C".to_string(),         // Plausible, classic for structured programming
            "Python".to_string(),    // Plausible, supports structured programming
            "JavaScript".to_string(),// Plausible, used in structured codebases
        ],
        0,
    ));

    category.add_question(Question::new(
        "MS/DOS is written in which language?".to_string(),
        vec![
            "C++".to_string(), // Correct
            "Assembly".to_string(), // Hard false
            "BASIC".to_string(), // Hard false
            "C".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "A program element that allows structuring of a program in a different way is called?".to_string(),
        vec![
            "Co-Routine".to_string(), // Correct
            "Sub-Routine".to_string(), // Hard false
            "Function".to_string(), // Hard false
            "Loop".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "A name given by Intel to high-speed MOS technology is called?".to_string(),
        vec![
            "HMOS".to_string(), // Correct
            "CMOS".to_string(), // Hard false
            "BIOS".to_string(), // Hard false
            "NMOS".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which country recently shut down the use of FM Radio?".to_string(),
        vec![
            "Norway".to_string(), // Correct
            "Sweden".to_string(), // Hard false
            "Finland".to_string(), // Hard false
            "Denmark".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does OSI stand for?".to_string(),
        vec![
            "Open Systems Interconnection (Layer 7)".to_string(), // Correct
            "Open Source Interface (Layer 7)".to_string(), // Hard false
            "Online Systems Integration (Layer 7)".to_string(), // Hard false
            "Open Secure Interface (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which OSI layer is responsible for routing data between different networks?".to_string(),
        vec![
            "Network Layer (Layer 3)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which layer of the OSI model is responsible for establishing, maintaining, and terminating communication sessions?".to_string(),
        vec![
            "Session Layer (Layer 5)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Presentation Layer (Layer 6)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which OSI layer handles the physical transmission of data over a medium?".to_string(),
        vec![
            "Physical Layer (Layer 1)".to_string(), // Correct
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which OSI layer is responsible for error detection and correction?".to_string(),
        vec![
            "Data Link Layer (Layer 2)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which OSI layer formats data and is responsible for data encryption and compression?".to_string(),
        vec![
            "Presentation Layer (Layer 6)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Session Layer (Layer 5)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "At which layer of the OSI model do HTTP, FTP, and SMTP operate?".to_string(),
        vec![
            "Application Layer (Layer 7)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which OSI layer is responsible for end-to-end communication and flow control?".to_string(),
        vec![
            "Transport Layer (Layer 4)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Physical Layer (Layer 1)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "In the OSI model, which layer is responsible for IP addressing?".to_string(),
        vec![
            "Network Layer (Layer 3)".to_string(), // Correct
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Application Layer (Layer 7)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following layers of the OSI model is responsible for converting digital signals to analog and vice versa?".to_string(),
        vec![
            "Physical Layer (Layer 1)".to_string(), // Correct
            "Data Link Layer (Layer 2)".to_string(), // Hard false
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Presentation Layer (Layer 6)".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "What is the size (in bits) of an IPv4 address?".to_string(),
        vec![
            "32 bits".to_string(), // Correct
            "64 bits".to_string(), // Hard false
            "128 bits".to_string(), // Hard false
            "16 bits".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which year was IPv6 officially released?".to_string(),
        vec![
            "1998".to_string(), // Correct
            "2001".to_string(), // Hard false
            "1994".to_string(), // Hard false
            "2005".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which IPv4 class is used for private addresses like 192.168.x.x?".to_string(),
        vec![
            "Class C".to_string(), // Correct
            "Class A".to_string(), // Hard false
            "Class B".to_string(), // Hard false
            "Class D".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the maximum number of hosts that can be supported on a subnet with a /24 prefix in IPv4?".to_string(),
        vec![
            "254 hosts".to_string(), // Correct
            "256 hosts".to_string(), // Hard false
            "128 hosts".to_string(), // Hard false
            "512 hosts".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the total number of possible IPv4 addresses?".to_string(),
        vec![
            "4.3 billion".to_string(), // Correct
            "2.1 billion".to_string(), // Hard false
            "8.6 billion".to_string(), // Hard false
            "1.5 billion".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is a valid IPv6 address format?".to_string(),
        vec![
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(), // Correct
            "192.168.0.1".to_string(), // Hard false
            "255.255.255.0".to_string(), // Hard false
            "3000:0db8:85a3:0000::0000".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which part of an IPv4 address represents the network portion?".to_string(),
        vec![
            "The first few bits, determined by the subnet mask".to_string(), // Correct
            "The last few bits, after the slash".to_string(), // Hard false
            "All of the address".to_string(), // Hard false
            "The middle section of the address".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does the IPv6 address block 2001:0db8::/32 represent?".to_string(),
        vec![
            "It is reserved for documentation purposes".to_string(), // Correct
            "It is used for local communication only".to_string(), // Hard false
            "It is a private network address".to_string(), // Hard false
            "It is the address for the default gateway".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does the CIDR notation /8 represent in terms of IPv4 subnetting?".to_string(),
        vec![
            "It represents 16,777,216 possible addresses".to_string(), // Correct
            "It represents 256 possible addresses".to_string(), // Hard false
            "It represents 1,024 possible addresses".to_string(), // Hard false
            "It represents 65,536 possible addresses".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which IPv6 feature helps avoid network address translation (NAT)?".to_string(),
        vec![
            "Auto-configuration (Stateless Address Autoconfiguration)".to_string(), // Correct
            "Private addressing".to_string(), // Hard false
            "IP Masquerading".to_string(), // Hard false
            "Dynamic Host Configuration Protocol (DHCP)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which type of address is used to identify a single device in IPv6?".to_string(),
        vec![
            "Unicast".to_string(), // Correct
            "Multicast".to_string(), // Hard false
            "Anycast".to_string(), // Hard false
            "Broadcast".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the purpose of the IPv6 link-local address?".to_string(),
        vec![
            "To allow devices on the same link to communicate".to_string(), // Correct
            "To identify a device across the entire network".to_string(), // Hard false
            "To enable NAT traversal".to_string(), // Hard false
            "To route packets across the internet".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which subnet mask is equivalent to the IPv4 CIDR prefix /16?".to_string(),
        vec![
            "255.255.0.0".to_string(), // Correct
            "255.255.255.0".to_string(), // Hard false
            "255.255.255.255".to_string(), // Hard false
            "255.255.255.224".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which IPv6 address is used to send a packet to multiple devices on a local network?".to_string(),
        vec![
            "Multicast address".to_string(), // Correct
            "Unicast address".to_string(), // Hard false
            "Anycast address".to_string(), // Hard false
            "Broadcast address".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the purpose of subnetting in IPv4 networking?".to_string(),
        vec![
            "To divide a large network into smaller, more manageable sub-networks".to_string(), // Correct
            "To increase the total number of available addresses".to_string(), // Hard false
            "To allow the use of private addresses only".to_string(), // Hard false
            "To combine multiple networks into one larger network".to_string(), // Hard false
        ],
        0,
    ));

 
    category.add_question(Question::new(
        "What is the length of a MAC address?".to_string(),
        vec![
            "48 bits".to_string(), // Correct
            "32 bits".to_string(), // Hard false
            "64 bits".to_string(), // Hard false
            "128 bits".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the format of a MAC address?".to_string(),
        vec![
            "6 pairs of hexadecimal digits".to_string(), // Correct
            "4 pairs of hexadecimal digits".to_string(), // Hard false
            "8 digits of binary numbers".to_string(), // Hard false
            "A series of numbers and letters separated by colons".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is true about a MAC address?".to_string(),
        vec![
            "It uniquely identifies network interfaces on a local network".to_string(), // Correct
            "It identifies the network protocol version".to_string(), // Hard false
            "It changes with each network connection".to_string(), // Hard false
            "It is used to route traffic on the internet".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which part of a MAC address identifies the manufacturer of the network interface?".to_string(),
        vec![
            "The first 3 bytes (OUI - Organizationally Unique Identifier)".to_string(), // Correct
            "The last 3 bytes".to_string(), // Hard false
            "The middle 2 bytes".to_string(), // Hard false
            "The entire address".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does MAC stand for in the context of networking?".to_string(),
        vec![
            "Media Access Control".to_string(), // Correct
            "Machine Access Code".to_string(), // Hard false
            "Modular Access Control".to_string(), // Hard false
            "Maximum Allowed Capacity".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which layer of the OSI model uses MAC addresses for communication?".to_string(),
        vec![
            "Data Link Layer (Layer 2)".to_string(), // Correct
            "Network Layer (Layer 3)".to_string(), // Hard false
            "Transport Layer (Layer 4)".to_string(), // Hard false
            "Session Layer (Layer 5)".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is a valid MAC address format?".to_string(),
        vec![
            "00:1A:2B:3C:4D:5E".to_string(), // Correct
            "192.168.1.1".to_string(), // Hard false
            "00-1A-2B-3C-4D-5E".to_string(), // Hard false
            "0x001A2B3C4D5E".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is a common way to represent a MAC address?".to_string(),
        vec![
            "Hexadecimal format with colons or hyphens".to_string(), // Correct
            "Decimal format with spaces".to_string(), // Hard false
            "Binary format with dashes".to_string(), // Hard false
            "Octal format with commas".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Can a MAC address be changed or spoofed?".to_string(),
        vec![
            "Yes, it can be spoofed through software".to_string(), // Correct
            "No, a MAC address is permanent".to_string(), // Hard false
            "Yes, but only through hardware modifications".to_string(), // Hard false
            "No, the MAC address is assigned by the router".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the primary purpose of a MAC address in networking?".to_string(),
        vec![
            "To identify devices on a local network".to_string(), // Correct
            "To route packets between networks".to_string(), // Hard false
            "To secure network connections".to_string(), // Hard false
            "To determine the IP address of a device".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does it mean if two devices have the same MAC address?".to_string(),
        vec![
            "It may cause a conflict on the network".to_string(), // Correct
            "It means they are connected to different networks".to_string(), // Hard false
            "It is a sign of network security".to_string(), // Hard false
            "It is completely fine as long as they use different IP addresses".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the function of the MAC address in Ethernet frames?".to_string(),
        vec![
            "It is used for identifying the source and destination devices".to_string(), // Correct
            "It is used to define the protocol used in the frame".to_string(), // Hard false
            "It is used to assign an IP address to the frame".to_string(), // Hard false
            "It is used to encrypt the data within the frame".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is true about a MAC address in a Wi-Fi network?".to_string(),
        vec![
            "It is used for identifying the wireless device on the network".to_string(), // Correct
            "It is used for assigning a channel to the network".to_string(), // Hard false
            "It determines the Wi-Fi encryption type".to_string(), // Hard false
            "It is used for determining the frequency band".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "In which range does the OUI (Organizationally Unique Identifier) of a MAC address lie?".to_string(),
        vec![
            "First 3 bytes".to_string(), // Correct
            "Last 3 bytes".to_string(), // Hard false
            "Middle 2 bytes".to_string(), // Hard false
            "Entire 6 bytes".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the range of MAC addresses used for locally administered addresses?".to_string(),
        vec![
            "Addresses where the first bit of the first byte is set to 1".to_string(), // Correct
            "Addresses where the last byte is reserved".to_string(), // Hard false
            "Addresses where the first byte is set to 0".to_string(), // Hard false
            "Addresses that end with FF".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "Which of the following is used for wired Ethernet connections?".to_string(),
        vec![
            "RJ45".to_string(), // Correct
            "USB-C".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
            "Thunderbolt".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the maximum theoretical speed of Wi-Fi 6 (802.11ax)?".to_string(),
        vec![
            "9.6 Gbps".to_string(), // Correct
            "1 Gbps".to_string(), // Hard false
            "5 Gbps".to_string(), // Hard false
            "40 Gbps".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which Bluetooth version introduced support for Low Energy (LE)?".to_string(),
        vec![
            "Bluetooth 4.0".to_string(), // Correct
            "Bluetooth 3.0".to_string(), // Hard false
            "Bluetooth 2.0".to_string(), // Hard false
            "Bluetooth 5.0".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the main purpose of an RJ45 connector?".to_string(),
        vec![
            "It is used for Ethernet networking".to_string(), // Correct
            "It is used for HDMI connections".to_string(), // Hard false
            "It is used for USB connections".to_string(), // Hard false
            "It is used for audio equipment".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the typical range of Bluetooth 5.0?".to_string(),
        vec![
            "Up to 240 meters".to_string(), // Correct
            "Up to 10 meters".to_string(), // Hard false
            "Up to 100 meters".to_string(), // Hard false
            "Up to 1 meter".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which Wi-Fi standard operates on the 2.4 GHz and 5 GHz bands?".to_string(),
        vec![
            "802.11ac".to_string(), // Correct
            "802.11b".to_string(), // Hard false
            "802.11n".to_string(), // Hard false
            "802.11a".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following connectors is commonly used for fiber optic cables?".to_string(),
        vec![
            "SC Connector".to_string(), // Correct
            "RJ45".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
            "USB-A".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does the term 'Bluetooth pairing' refer to?".to_string(),
        vec![
            "Establishing a secure connection between Bluetooth devices".to_string(), // Correct
            "Charging Bluetooth devices".to_string(), // Hard false
            "Setting up a Wi-Fi network".to_string(), // Hard false
            "Updating the Bluetooth version on a device".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the frequency range of Wi-Fi 5 (802.11ac)?".to_string(),
        vec![
            "5 GHz".to_string(), // Correct
            "2.4 GHz".to_string(), // Hard false
            "900 MHz".to_string(), // Hard false
            "1.2 GHz".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which version of Bluetooth allows for faster data transfer rates of up to 3 Mbps?".to_string(),
        vec![
            "Bluetooth 2.0".to_string(), // Correct
            "Bluetooth 3.0".to_string(), // Hard false
            "Bluetooth 4.0".to_string(), // Hard false
            "Bluetooth 5.0".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which connection type is commonly used for long-distance, high-speed internet connections in homes and businesses?".to_string(),
        vec![
            "Fiber Optic".to_string(), // Correct
            "Wi-Fi".to_string(), // Hard false
            "Ethernet (RJ45)".to_string(), // Hard false
            "Bluetooth".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the maximum range of Wi-Fi 6 (802.11ax)?".to_string(),
        vec![
            "Up to 300 meters".to_string(), // Correct
            "Up to 100 meters".to_string(), // Hard false
            "Up to 500 meters".to_string(), // Hard false
            "Up to 50 meters".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is true about RJ45 connectors?".to_string(),
        vec![
            "Used for Ethernet networking over twisted pair cables".to_string(), // Correct
            "Used for HDMI video connections".to_string(), // Hard false
            "Used for connecting audio equipment".to_string(), // Hard false
            "Used for USB connections".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the name of the Wi-Fi technology that allows devices to communicate without a traditional router?".to_string(),
        vec![
            "Wi-Fi Direct".to_string(), // Correct
            "Wi-Fi Mesh".to_string(), // Hard false
            "Wi-Fi Hub".to_string(), // Hard false
            "Wi-Fi Bridge".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following connection types is commonly used for connecting devices like printers and keyboards to a computer without wires?".to_string(),
        vec![
            "Bluetooth".to_string(), // Correct
            "RJ45".to_string(), // Hard false
            "USB-C".to_string(), // Hard false
            "HDMI".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "Which routing protocol is used in most modern internet routing?".to_string(), 
        vec![
            "BGP".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "IGRP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does the term 'dynamic routing' refer to?".to_string(), 
        vec![
            "Routes that are automatically updated and maintained by routers".to_string(), // Correct
            "Manually configured routes that don't change".to_string(), // Hard false
            "Routes that are static and need to be manually configured".to_string(), // Hard false
            "Routes that are used only during initial setup".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol is mainly used for routing within a single autonomous system?".to_string(), 
        vec![
            "OSPF".to_string(), // Correct
            "BGP".to_string(), // Hard false
            "RIP".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following devices is responsible for directing traffic within a local network?".to_string(), 
        vec![
            "Router".to_string(), // Correct
            "Switch".to_string(), // Hard false
            "Hub".to_string(), // Hard false
            "Modem".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which routing protocol uses a distance-vector method?".to_string(), 
        vec![
            "RIP".to_string(), // Correct
            "OSPF".to_string(), // Hard false
            "BGP".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What does BGP stand for?".to_string(), 
        vec![
            "Border Gateway Protocol".to_string(), // Correct
            "Binary Gateway Protocol".to_string(), // Hard false
            "Basic Gateway Protocol".to_string(), // Hard false
            "Best Gateway Protocol".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the main role of a default gateway in a network?".to_string(), 
        vec![
            "To route traffic between different networks".to_string(), // Correct
            "To connect devices within a local network".to_string(), // Hard false
            "To assign IP addresses to devices".to_string(), // Hard false
            "To filter incoming and outgoing traffic".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which protocol is used for routing between different autonomous systems?".to_string(), 
        vec![
            "BGP".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "OSPF".to_string(), // Hard false
            "EIGRP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which type of routing protocol is OSPF?".to_string(), 
        vec![
            "Link-State".to_string(), // Correct
            "Distance-Vector".to_string(), // Hard false
            "Hybrid".to_string(), // Hard false
            "Path-Vector".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the primary advantage of using OSPF over RIP?".to_string(), 
        vec![
            "Faster convergence time".to_string(), // Correct
            "More secure communication".to_string(), // Hard false
            "Lower resource usage".to_string(), // Hard false
            "Better compatibility with older routers".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is true about the routing protocol RIP?".to_string(), 
        vec![
            "It uses hop count as a metric".to_string(), // Correct
            "It uses bandwidth as a metric".to_string(), // Hard false
            "It supports larger networks than OSPF".to_string(), // Hard false
            "It has faster convergence than OSPF".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which router protocol is considered more efficient for large-scale networks?".to_string(), 
        vec![
            "OSPF".to_string(), // Correct
            "RIP".to_string(), // Hard false
            "BGP".to_string(), // Hard false
            "IGRP".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is a key feature of EIGRP?".to_string(), 
        vec![
            "It uses Diffusing Update Algorithm (DUAL)".to_string(), // Correct
            "It uses link-state information".to_string(), // Hard false
            "It relies on static routes".to_string(), // Hard false
            "It does not support variable-length subnet masking".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the primary difference between static and dynamic routing?".to_string(), 
        vec![
            "Static routing requires manual configuration, dynamic routing adapts automatically".to_string(), // Correct
            "Static routing is more secure than dynamic routing".to_string(), // Hard false
            "Static routing is used only for small networks".to_string(), // Hard false
            "Dynamic routing is not used in large networks".to_string(), // Hard false
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is the main function of a routing table?".to_string(), 
        vec![
            "To store the best paths to destination networks".to_string(), // Correct
            "To filter incoming and outgoing network traffic".to_string(), // Hard false
            "To assign IP addresses to devices".to_string(), // Hard false
            "To store DNS information for domain resolution".to_string(), // Hard false
        ],
        0,
    ));


    category.add_question(Question::new(
        "Who developed the first email system used on ARPANET?".to_string(),
        vec![
            "Ray Tomlinson".to_string(), // Correct
            "Vint Cerf".to_string(),
            "Jon Postel".to_string(),
            "Leonard Kleinrock".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was the name of the first known computer worm to spread over the internet?".to_string(),
        vec![
            "Morris Worm".to_string(), // Correct
            "ILOVEYOU".to_string(),
            "Conficker".to_string(),
            "Code Red".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which obscure IBM product was known as the 'RS/6000 SP' and inspired the supercomputer in the movie 'Deep Blue'?".to_string(),
        vec![
            "IBM Scalable POWERparallel System".to_string(), // Correct
            "IBM Blue Gene".to_string(),
            "IBM Quantum System One".to_string(),
            "IBM System/360 Model 91".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which programming language was originally developed to control industrial machines like CNCs?".to_string(),
        vec![
            "G-Code".to_string(), // Correct
            "Forth".to_string(),
            "COBOL".to_string(),
            "Ada".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which company first introduced a consumer laptop with a trackpoint (pointing stick) in its keyboard?".to_string(),
        vec![
            "IBM".to_string(), // Correct
            "Toshiba".to_string(),
            "Compaq".to_string(),
            "Dell".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was the name of the first computer to use silicon transistors instead of vacuum tubes?".to_string(),
        vec![
            "TX-0".to_string(), // Correct
            "UNIVAC I".to_string(),
            "EDSAC".to_string(),
            "ENIAC".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is credited with developing the first commercial database system, IMS (Information Management System)?".to_string(),
        vec![
            "IBM".to_string(), // Correct
            "Oracle".to_string(),
            "Honeywell".to_string(),
            "DEC".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was the name of the first portable computer, introduced in 1981?".to_string(),
        vec![
            "Osborne 1".to_string(), // Correct
            "IBM PC 5150".to_string(),
            "Compaq Portable".to_string(),
            "Apple Macintosh".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which bizarre device used in the 1970s was known as the 'Honeywell Kitchen Computer'?".to_string(),
        vec![
            "Honeywell H316".to_string(), // Correct
            "Honeywell Model 3".to_string(),
            "Honeywell Multics".to_string(),
            "Honeywell Mark III".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which ancient programming language is still used to control air traffic systems worldwide?".to_string(),
        vec![
            "JOVIAL".to_string(), // Correct
            "COBOL".to_string(),
            "Ada".to_string(),
            "Fortran".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who was the inventor of the VESA mount standard?".to_string(),
        vec![
            "Erik Anderson".to_string(), 
            "Lars Thomsen".to_string(),
            "Bill Lempesis".to_string(), // Correct
            "John H. VESA".to_string(), // Trick answer
        ],
        2,
    ));

    category.add_question(Question::new(
        "Which company first introduced the PCI Express (PCIe) standard?".to_string(),
        vec![
            "Intel".to_string(), // Correct
            "IBM".to_string(),
            "AMD".to_string(),
            "NVIDIA".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who invented the first graphical user interface (GUI)?".to_string(),
        vec![
            "Douglas Engelbart".to_string(),
            "Alan Kay".to_string(), // Correct
            "Steve Jobs".to_string(),
            "Xerox PARC team".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who developed the concept of the first trackball device?".to_string(),
        vec![
            "Ralph Benjamin".to_string(), // Correct
            "Douglas Engelbart".to_string(),
            "Clifford Berry".to_string(),
            "Ivan Sutherland".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which company developed the first solid-state drive (SSD)?".to_string(),
        vec![
            "SanDisk".to_string(),
            "IBM".to_string(),
            "StorageTek".to_string(), // Correct
            "Toshiba".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Which programming language was developed by IBM for artificial intelligence in 1956?".to_string(),
        vec![
            "FORTRAN".to_string(),
            "LISP".to_string(),
            "IPL (Information Processing Language)".to_string(), // Correct
            "ALGOL".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Who designed the first computer virus intended for testing purposes?".to_string(),
        vec![
            "Fred Cohen".to_string(),
            "John von Neumann".to_string(), // Correct
            "Robert Tappan Morris".to_string(),
            "Mark Russinovich".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who created the ZIP file compression format?".to_string(),
        vec![
            "Phil Katz".to_string(), // Correct
            "Ross Williams".to_string(),
            "Gary Kildall".to_string(),
            "Jean-loup Gailly".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What was the first computer mouse made of?".to_string(),
        vec![
            "Plastic".to_string(),
            "Wood".to_string(), // Correct
            "Metal".to_string(),
            "Bakelite".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Which company developed the first color computer monitor?".to_string(),
        vec![
            "Sony".to_string(),
            "IBM".to_string(),
            "RCA".to_string(), // Correct
            "Compaq".to_string(),
        ],
        2,
    ));


    category.add_question(Question::new(
        "Who is considered the father of the modern computer?".to_string(),
        vec![
            "Alan Turing".to_string(),
            "Charles Babbage".to_string(), // Correct
            "John von Neumann".to_string(),
            "Steve Jobs".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who is known as the father of artificial intelligence?".to_string(),
        vec![
            "John McCarthy".to_string(), // Correct
            "Alan Turing".to_string(),
            "Marvin Minsky".to_string(),
            "Herbert Simon".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who developed the first compiler?".to_string(),
        vec![
            "Grace Hopper".to_string(), // Correct
            "Ada Lovelace".to_string(),
            "Alan Kay".to_string(),
            "Donald Knuth".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which personality is often associated with the invention of the Internet?".to_string(),
        vec![
            "Tim Berners-Lee".to_string(),
            "Vint Cerf".to_string(), // Correct
            "Robert Kahn".to_string(),
            "Linus Torvalds".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who invented the World Wide Web?".to_string(),
        vec![
            "Tim Berners-Lee".to_string(), // Correct
            "Marc Andreessen".to_string(),
            "Bill Gates".to_string(),
            "Steve Wozniak".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Which IT personality founded Microsoft?".to_string(),
        vec![
            "Paul Allen".to_string(),
            "Steve Jobs".to_string(),
            "Bill Gates".to_string(), // Correct
            "Larry Page".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Who developed the Linux operating system?".to_string(),
        vec![
            "Dennis Ritchie".to_string(),
            "Ken Thompson".to_string(),
            "Linus Torvalds".to_string(), // Correct
            "Richard Stallman".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Who is considered the first programmer in history?".to_string(),
        vec![
            "Grace Hopper".to_string(),
            "Ada Lovelace".to_string(), // Correct
            "Margaret Hamilton".to_string(),
            "Hedy Lamarr".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who developed the concept of relational databases?".to_string(),
        vec![
            "Larry Ellison".to_string(),
            "Edgar F. Codd".to_string(), // Correct
            "James Gosling".to_string(),
            "Donald D. Chamberlin".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who is regarded as the creator of Bitcoin?".to_string(),
        vec![
            "Vitalik Buterin".to_string(),
            "Hal Finney".to_string(),
            "Satoshi Nakamoto".to_string(), // Correct
            "Gavin Andresen".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Who invented the programming language C?".to_string(),
        vec![
            "Brian Kernighan".to_string(),
            "Ken Thompson".to_string(),
            "Dennis Ritchie".to_string(), // Correct
            "Bjarne Stroustrup".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Who played a major role in developing UNIX?".to_string(),
        vec![
            "Dennis Ritchie".to_string(),
            "Ken Thompson".to_string(), // Correct
            "Linus Torvalds".to_string(),
            "Alan Turing".to_string(),
        ],
        1,
    ));

    category.add_question(Question::new(
        "Who developed the first version of Java?".to_string(),
        vec![
            "James Gosling".to_string(), // Correct
            "Guido van Rossum".to_string(),
            "Brendan Eich".to_string(),
            "Rasmus Lerdorf".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who invented the Python programming language?".to_string(),
        vec![
            "James Gosling".to_string(),
            "Larry Wall".to_string(),
            "Guido van Rossum".to_string(), // Correct
            "Bjarne Stroustrup".to_string(),
        ],
        2,
    ));

    category.add_question(Question::new(
        "Which personality co-founded Apple?".to_string(),
        vec![
            "Steve Jobs".to_string(),
            "Steve Wozniak".to_string(),
            "Ronald Wayne".to_string(),
            "All of the above".to_string(), // Correct
        ],
        3,
    ));

    category.add_question(Question::new(
        "Who is known as the creator of the algorithm used for Google search?".to_string(),
        vec![
            "Larry Page".to_string(), // Correct
            "Sergey Brin".to_string(),
            "Jeff Dean".to_string(),
            "Peter Norvig".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who developed the first email application?".to_string(),
        vec![
            "Ray Tomlinson".to_string(), // Correct
            "Vint Cerf".to_string(),
            "Paul Baran".to_string(),
            "Jon Postel".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who developed the foundations of information encryption?".to_string(),
        vec![
            "Claude Shannon".to_string(), // Correct
            "Alan Turing".to_string(),
            "Whitfield Diffie".to_string(),
            "Ralph Merkle".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who developed the first version of Ruby on Rails?".to_string(),
        vec![
            "David Heinemeier Hansson".to_string(), // Correct
            "Matz (Yukihiro Matsumoto)".to_string(),
            "Martin Fowler".to_string(),
            "Kent Beck".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "Who is considered the founder of the open-source movement?".to_string(),
        vec![
            "Linus Torvalds".to_string(),
            "Richard Stallman".to_string(), // Correct
            "Bruce Perens".to_string(),
            "Eric S. Raymond".to_string(),
        ],
        1,
    ));











    // Kopieren Sie hier einfach alle weiteren IT-Fragen aus Ihrer data.rs,
    // der Aufruf bleibt category.add_question(Question::new(...))
}
