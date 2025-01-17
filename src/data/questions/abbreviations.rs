use crate::models::Category;
use crate::models::Question;

pub fn add_abbreviation_questions(category: &mut Category) {
    category.add_question(Question::new(
        "What does ASAP stand for?".to_string(),
        vec!["As Soon As Possible".to_string(), "As Simple As Possible".to_string()],
        0,
    ));

    category.add_question(question::new(
        "what does ceo stand for?".to_string(),
        vec!["chief executive officer".to_string(), "central executive officer".to_string()],
        0,
    ));
    category.add_question(question::new(
        "what does faq stand for?".to_string(),
        vec!["frequently asked questions".to_string(), "frequently answered questions".to_string()],
        0,
    ));

    category.add_question(question::new(
        "what does api stand for?".to_string(),
        vec![
            "application programming interface".to_string(), // correct
            "advanced peripheral integration".to_string(),    // hard false
            "automatic process invocation".to_string(),      // hard false
            "analytical programming interface".to_string(),  // hard false
        ],
        0, // correct answer index
    ));

    category.add_question(question::new(
        "what does ascii stand for?".to_string(),
        vec![
            "american standard code for information interchange".to_string(), // correct
            "automated system code for interface integration".to_string(),    // hard false
            "advanced standard for code interoperation".to_string(),          // hard false
            "architectural syntax control information interface".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does bios stand for?".to_string(),
        vec![
            "basic input/output system".to_string(), // correct
            "binary integrated operating system".to_string(), // hard false
            "backend infrastructure optimization software".to_string(), // hard false
            "biological input output source".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does bgp stand for?".to_string(),
        vec![
            "border gateway protocol".to_string(), // correct
            "binary gateway process".to_string(), // hard false
            "bridge group protocol".to_string(), // hard false
            "boundary graphical processing".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does cdn stand for?".to_string(),
        vec![
            "content delivery network".to_string(), // correct
            "central data network".to_string(),     // hard false
            "cascading distribution node".to_string(), // hard false
            "clustered data network".to_string(),   // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does cpu stand for?".to_string(),
        vec![
            "central processing unit".to_string(), // correct
            "core peripheral unit".to_string(),   // hard false
            "clustered processing utility".to_string(), // hard false
            "compute program unit".to_string(),   // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does css stand for?".to_string(),
        vec![
            "cascading style sheets".to_string(), // correct
            "central system script".to_string(), // hard false
            "custom styling syntax".to_string(), // hard false
            "controlled style sheets".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does db stand for?".to_string(),
        vec![
            "database".to_string(), // correct
            "data block".to_string(), // hard false
            "distributed buffer".to_string(), // hard false
            "dynamic bytecode".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does dns stand for?".to_string(),
        vec![
            "domain name system".to_string(), // correct
            "distributed network service".to_string(), // hard false
            "dynamic node synchronization".to_string(), // hard false
            "domain networking suite".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does dos stand for?".to_string(),
        vec![
            "disk operating system".to_string(), // correct
            "data operating solution".to_string(), // hard false
            "dynamic operating system".to_string(), // hard false
            "distributed optimization software".to_string(), // hard false
        ],
        0,
    ));
    category.add_question(question::new(
        "what does ansi stand for?".to_string(),
        vec![
            "american national standards institute".to_string(), // correct
            "advanced network systems interface".to_string(),    // hard false
            "association of national system integrators".to_string(), // hard false
            "automated numerical standards interface".to_string(), // hard false
        ],
        0, // correct answer index
    ));

    category.add_question(question::new(
        "what does arp stand for?".to_string(),
        vec![
            "address resolution protocol".to_string(), // correct
            "application routing protocol".to_string(), // hard false
            "advanced redirection process".to_string(), // hard false
            "automated request protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does atm stand for?".to_string(),
        vec![
            "asynchronous transfer mode".to_string(), // correct
            "automated teller machine".to_string(), // trick answer
            "active transmission matrix".to_string(), // hard false
            "advanced transfer management".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does bfs stand for?".to_string(),
        vec![
            "breadth-first search".to_string(), // correct
            "binary file system".to_string(),  // hard false
            "backup file synchronization".to_string(), // hard false
            "basic file search".to_string(),   // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does bt stand for?".to_string(),
        vec![
            "bluetooth".to_string(), // correct
            "binary transfer".to_string(), // hard false
            "basic telecommunication".to_string(), // hard false
            "broadband transmission".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ci/cd stand for?".to_string(),
        vec![
            "continuous integration/continuous deployment".to_string(), // correct
            "code integration/code development".to_string(), // hard false
            "continuous improvement/controlled delivery".to_string(), // hard false
            "critical infrastructure/cloud deployment".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does crc stand for?".to_string(),
        vec![
            "cyclic redundancy check".to_string(), // correct
            "critical redundancy check".to_string(), // hard false
            "controlled routing cipher".to_string(), // hard false
            "code redundancy calculation".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does dhcp stand for?".to_string(),
        vec![
            "dynamic host configuration protocol".to_string(), // correct
            "distributed host communication protocol".to_string(), // hard false
            "data handling configuration protocol".to_string(), // hard false
            "dynamic hypertext control protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does dnssec stand for?".to_string(),
        vec![
            "dns security extensions".to_string(), // correct
            "distributed node security system".to_string(), // hard false
            "dynamic name service encryption control".to_string(), // hard false
            "domain name system encrypted channel".to_string(), // hard false
        ],
        0,
    ));
    category.add_question(question::new(
        "what does dr stand for?".to_string(),
        vec![
            "disaster recovery".to_string(), // correct
            "data redundancy".to_string(),  // hard false
            "distributed resources".to_string(), // hard false
            "dynamic replication".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does fat stand for?".to_string(),
        vec![
            "file allocation table".to_string(), // correct
            "fast access table".to_string(), // hard false
            "file attribute tracker".to_string(), // hard false
            "flexible archive tool".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does gpu stand for?".to_string(),
        vec![
            "graphics processing unit".to_string(), // correct
            "general purpose unit".to_string(), // hard false
            "graphics protocol utility".to_string(), // hard false
            "global performance unit".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does html stand for?".to_string(),
        vec![
            "hypertext markup language".to_string(), // correct
            "high-tech metadata layer".to_string(), // hard false
            "hypertext module language".to_string(), // hard false
            "hierarchical template modeling language".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ipsec stand for?".to_string(),
        vec![
            "internet protocol security".to_string(), // correct
            "ip secure protocol".to_string(), // hard false
            "internet privacy security".to_string(), // hard false
            "integrated protocol security".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does jvm stand for?".to_string(),
        vec![
            "java virtual machine".to_string(), // correct
            "just-in-time virtual manager".to_string(), // hard false
            "java version manager".to_string(), // hard false
            "java variable management".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does nfs stand for?".to_string(),
        vec![
            "network file system".to_string(), // correct
            "node file storage".to_string(), // hard false
            "network file synchronization".to_string(), // hard false
            "net folder sharing".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does oop stand for?".to_string(),
        vec![
            "object-oriented programming".to_string(), // correct
            "optimal operations protocol".to_string(), // hard false
            "object orchestration process".to_string(), // hard false
            "operational output programming".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does raid stand for?".to_string(),
        vec![
            "redundant array of independent disks".to_string(), // correct
            "rapid access internal disks".to_string(), // hard false
            "remote array of integrated devices".to_string(), // hard false
            "redundant access input data".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does smtp stand for?".to_string(),
        vec![
            "simple mail transfer protocol".to_string(), // correct
            "secure mail transmission protocol".to_string(), // hard false
            "standard message transport protocol".to_string(), // hard false
            "simple messaging transport procedure".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does wlan stand for?".to_string(),
        vec![
            "wireless local area network".to_string(), // correct
            "wide local access network".to_string(),  // hard false
            "wireless linked access node".to_string(), // hard false
            "wide loop area network".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does www stand for?".to_string(),
        vec![
            "world wide web".to_string(), // correct
            "wide world web".to_string(), // hard false
            "web without wires".to_string(), // hard false
            "wide wireless web".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does xml stand for?".to_string(),
        vec![
            "extensible markup language".to_string(), // correct
            "extended metadata layer".to_string(), // hard false
            "exclusive model language".to_string(), // hard false
            "exchange markup layout".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does yaml stand for?".to_string(),
        vec![
            "yaml ain't markup language".to_string(), // correct
            "yet another markup language".to_string(), // hard false
            "your accessible markup layout".to_string(), // hard false
            "you always make layouts".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does zip stand for?".to_string(),
        vec![
            "zone improvement plan".to_string(), // correct
            "zero integration protocol".to_string(), // hard false
            "zipped internet protocol".to_string(), // hard false
            "zone interchange package".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does acl stand for?".to_string(),
        vec![
            "access control list".to_string(), // correct
            "advanced configuration layer".to_string(), // hard false
            "application connection log".to_string(), // hard false
            "authorized control ledger".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does aes stand for?".to_string(),
        vec![
            "advanced encryption standard".to_string(), // correct
            "automated encryption system".to_string(), // hard false
            "advanced encoding schema".to_string(), // hard false
            "authenticated encryption service".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ai stand for?".to_string(),
        vec![
            "artificial intelligence".to_string(), // correct
            "automated integration".to_string(), // hard false
            "augmented interpretation".to_string(), // hard false
            "advanced interaction".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ajax stand for?".to_string(),
        vec![
            "asynchronous javascript and xml".to_string(), // correct
            "advanced javascript and xml".to_string(), // hard false
            "automated javascript application".to_string(), // hard false
            "assembled javascript and xhtml".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does udp stand for?".to_string(),
        vec![
            "user datagram protocol".to_string(), // correct
            "universal data packet".to_string(), // hard false
            "user data protocol".to_string(), // hard false
            "unified datagram processing".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does url stand for?".to_string(),
        vec![
            "uniform resource locator".to_string(), // correct
            "universal reference link".to_string(), // hard false
            "uniform retrieval location".to_string(), // hard false
            "unique resource link".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does usb stand for?".to_string(),
        vec![
            "universal serial bus".to_string(), // correct
            "unified system bus".to_string(), // hard false
            "universal storage bridge".to_string(), // hard false
            "universal signal bus".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does uuid stand for?".to_string(),
        vec![
            "universally unique identifier".to_string(), // correct
            "universal unit identifier".to_string(), // hard false
            "unique universal id".to_string(), // hard false
            "unified unique index".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does vlan stand for?".to_string(),
        vec![
            "virtual local area network".to_string(), // correct
            "virtual linked access node".to_string(), // hard false
            "virtual loopback area network".to_string(), // hard false
            "virtual local access network".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does vm stand for?".to_string(),
        vec![
            "virtual machine".to_string(), // correct
            "virtual module".to_string(), // hard false
            "verified machine".to_string(), // hard false
            "virtual memory".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does vpn stand for?".to_string(),
        vec![
            "virtual private network".to_string(), // correct
            "virtual public node".to_string(), // hard false
            "verified private network".to_string(), // hard false
            "virtual proxy network".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does vram stand for?".to_string(),
        vec![
            "video random-access memory".to_string(), // correct
            "virtual random-access memory".to_string(), // hard false
            "video read-access memory".to_string(), // hard false
            "verified random-access module".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does wan stand for?".to_string(),
        vec![
            "wide area network".to_string(), // correct
            "wireless access node".to_string(), // hard false
            "wide access network".to_string(), // hard false
            "wireless area network".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does wifi stand for?".to_string(),
        vec![
            "wireless fidelity".to_string(), // correct
            "wide fidelity".to_string(), // hard false
            "wireless frequency".to_string(), // hard false
            "wide frequency interface".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does sdk stand for?".to_string(),
        vec![
            "software development kit".to_string(), // correct
            "system design kit".to_string(),       // hard false
            "standard deployment kit".to_string(), // hard false
            "software deployment kernel".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does smtp stand for?".to_string(),
        vec![
            "simple mail transfer protocol".to_string(), // correct
            "secure mail transmission protocol".to_string(), // hard false
            "system mail transfer protocol".to_string(), // hard false
            "simple messaging transport process".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does snmp stand for?".to_string(),
        vec![
            "simple network management protocol".to_string(), // correct
            "secure network messaging protocol".to_string(), // hard false
            "system network management process".to_string(), // hard false
            "standard network management protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does sql stand for?".to_string(),
        vec![
            "structured query language".to_string(), // correct
            "simple query language".to_string(), // hard false
            "secure query language".to_string(), // hard false
            "standard query logic".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ssh stand for?".to_string(),
        vec![
            "secure shell".to_string(), // correct
            "simple system host".to_string(), // hard false
            "secure socket host".to_string(), // hard false
            "system shell handler".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ssl stand for?".to_string(),
        vec![
            "secure sockets layer".to_string(), // correct
            "secure socket link".to_string(), // hard false
            "standard security layer".to_string(), // hard false
            "secure server layer".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ssd stand for?".to_string(),
        vec![
            "solid-state drive".to_string(), // correct
            "standard storage device".to_string(), // hard false
            "solid storage disk".to_string(), // hard false
            "static storage device".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does tcp stand for?".to_string(),
        vec![
            "transmission control protocol".to_string(), // correct
            "transfer communication protocol".to_string(), // hard false
            "transport control process".to_string(), // hard false
            "transmission communication path".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does tls stand for?".to_string(),
        vec![
            "transport layer security".to_string(), // correct
            "transmission link security".to_string(), // hard false
            "transport link system".to_string(), // hard false
            "trusted layer security".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does tld stand for?".to_string(),
        vec![
            "top-level domain".to_string(), // correct
            "transport layer domain".to_string(), // hard false
            "trusted link domain".to_string(), // hard false
            "transmission level directory".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does pwm stand for?".to_string(),
        vec![
            "pulse width modulation".to_string(), // correct
            "power wave modulation".to_string(), // hard false
            "pulse wave modulator".to_string(), // hard false
            "power width manager".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ram stand for?".to_string(),
        vec![
            "random-access memory".to_string(), // correct
            "read-only memory".to_string(), // hard false
            "rapid access module".to_string(), // hard false
            "random allocation memory".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rdp stand for?".to_string(),
        vec![
            "remote desktop protocol".to_string(), // correct
            "resource data protocol".to_string(), // hard false
            "remote data process".to_string(), // hard false
            "remote device path".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rest stand for?".to_string(),
        vec![
            "representational state transfer".to_string(), // correct
            "remote synchronous transfer".to_string(), // hard false
            "resource state transport".to_string(), // hard false
            "representational service transfer".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rom stand for?".to_string(),
        vec![
            "read-only memory".to_string(), // correct
            "random-only memory".to_string(), // hard false
            "resource optimization module".to_string(), // hard false
            "read-on module".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rpc stand for?".to_string(),
        vec![
            "remote procedure call".to_string(), // correct
            "resource process communication".to_string(), // hard false
            "remote process command".to_string(), // hard false
            "rapid process communication".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rss stand for?".to_string(),
        vec![
            "really simple syndication".to_string(), // correct
            "resource syndication service".to_string(), // hard false
            "rapid subscription service".to_string(), // hard false
            "remote syndication system".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does rtos stand for?".to_string(),
        vec![
            "real-time operating system".to_string(), // correct
            "remote task operating system".to_string(), // hard false
            "real-time optimization system".to_string(), // hard false
            "rapid task optimization software".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does saas stand for?".to_string(),
        vec![
            "software as a service".to_string(), // correct
            "system as a service".to_string(), // hard false
            "storage as a service".to_string(), // hard false
            "software application subscription".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does san stand for?".to_string(),
        vec![
            "storage area network".to_string(), // correct
            "system area network".to_string(), // hard false
            "secure access network".to_string(), // hard false
            "shared application node".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ml stand for?".to_string(),
        vec![
            "machine learning".to_string(), // correct
            "markup language".to_string(), // hard false
            "matrix logic".to_string(), // hard false
            "modular learning".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does nat stand for?".to_string(),
        vec![
            "network address translation".to_string(), // correct
            "node allocation table".to_string(), // hard false
            "network allocation tracker".to_string(), // hard false
            "node address translator".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does nfc stand for?".to_string(),
        vec![
            "near field communication".to_string(), // correct
            "network file communication".to_string(), // hard false
            "near frequency connection".to_string(), // hard false
            "network frequency controller".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does os stand for?".to_string(),
        vec![
            "operating system".to_string(), // correct
            "operational software".to_string(), // hard false
            "online service".to_string(), // hard false
            "open source".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does paas stand for?".to_string(),
        vec![
            "platform as a service".to_string(), // correct
            "product as a service".to_string(), // hard false
            "protocol as a service".to_string(), // hard false
            "process as a service".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does pcie stand for?".to_string(),
        vec![
            "peripheral component interconnect express".to_string(), // correct
            "peripheral connection interface express".to_string(), // hard false
            "protocol component integration express".to_string(), // hard false
            "peripheral control integration express".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does pdf stand for?".to_string(),
        vec![
            "portable document format".to_string(), // correct
            "personal data file".to_string(), // hard false
            "public data framework".to_string(), // hard false
            "protocol document file".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does php stand for?".to_string(),
        vec![
            "php: hypertext preprocessor".to_string(), // correct
            "programming hypertext protocol".to_string(), // hard false
            "precompiled hypertext processor".to_string(), // hard false
            "protocol hypertext parser".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does pop3 stand for?".to_string(),
        vec![
            "post office protocol 3".to_string(), // correct
            "protocol of posts 3".to_string(), // hard false
            "post operational protocol 3".to_string(), // hard false
            "processing of posts 3".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does psu stand for?".to_string(),
        vec![
            "power supply unit".to_string(), // correct
            "power system unit".to_string(), // hard false
            "primary supply unit".to_string(), // hard false
            "peripheral supply unit".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does dram stand for?".to_string(),
        vec![
            "dynamic random-access memory".to_string(), // correct
            "dynamic read-access memory".to_string(), // hard false
            "direct random-access memory".to_string(), // hard false
            "dual random-access memory".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does erp stand for?".to_string(),
        vec![
            "enterprise resource planning".to_string(), // correct
            "enterprise resource processor".to_string(), // hard false
            "external resource planning".to_string(), // hard false
            "enterprise retrieval process".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ftp stand for?".to_string(),
        vec![
            "file transfer protocol".to_string(), // correct
            "file transport protocol".to_string(), // hard false
            "fast transfer protocol".to_string(), // hard false
            "file technical protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does gui stand for?".to_string(),
        vec![
            "graphical user interface".to_string(), // correct
            "graphical universal interface".to_string(), // hard false
            "global user interface".to_string(), // hard false
            "general user input".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does http stand for?".to_string(),
        vec![
            "hypertext transfer protocol".to_string(), // correct
            "hyper terminal transfer protocol".to_string(), // hard false
            "hypertext transmission protocol".to_string(), // hard false
            "hyper transfer protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does https stand for?".to_string(),
        vec![
            "hypertext transfer protocol secure".to_string(), // correct
            "hypertext transmission protocol secure".to_string(), // hard false
            "hyper terminal transfer protocol secure".to_string(), // hard false
            "hyper transfer protocol with security".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ide stand for?".to_string(),
        vec![
            "integrated development environment".to_string(), // correct
            "interactive development environment".to_string(), // hard false
            "integrated data environment".to_string(), // hard false
            "international development environment".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ip stand for?".to_string(),
        vec![
            "internet protocol".to_string(), // correct
            "internet processor".to_string(), // hard false
            "internal protocol".to_string(), // hard false
            "international protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ipv4 stand for?".to_string(),
        vec![
            "internet protocol version 4".to_string(), // correct
            "internet processor version 4".to_string(), // hard false
            "internal protocol version 4".to_string(), // hard false
            "internet protocol version four".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does ipv6 stand for?".to_string(),
        vec![
            "internet protocol version 6".to_string(), // correct
            "internet processor version 6".to_string(), // hard false
            "internal protocol version 6".to_string(), // hard false
            "internet protocol version six".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does soap stand for?".to_string(),
        vec![
            "simple object access protocol".to_string(), // correct
            "simple online access protocol".to_string(), // hard false
            "structured object access protocol".to_string(), // hard false
            "secure object access protocol".to_string(), // hard false
        ],
        0,
    ));

    category.add_question(question::new(
        "what does vnc stand for?".to_string(),
        vec![
            "virtual network computing".to_string(), // correct
            "virtual node computing".to_string(), // hard false
            "virtual network communication".to_string(), // hard false
            "virtual network controller".to_string(), // hard false
        ],
        0,
    ));

    // Kopieren Sie hier einfach alle weiteren Abkürzungs-Fragen aus Ihrer data.rs,
    // der Aufruf bleibt category.add_question(Question::new(...))
}
