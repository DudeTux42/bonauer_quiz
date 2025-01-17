use crate::models::Category;
use crate::models::Question;

pub fn add_math_questions(category: &mut Category) {
    category.add_question(Question::new(
        "What is 2 + 2?".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
        3,
    ));

    category.add_question(Question::new(
        "What is 3 * 3?".to_string(),
        vec!["6".to_string(), "9".to_string(), "12".to_string()],
        1,
    ));


    category.add_question(Question::new(
        "What is 2 + 2?".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
        3,
    ));
    category.add_question(Question::new(
        "What is 3 * 3?".to_string(),
        vec!["6".to_string(), "9".to_string(), "12".to_string()],
        1,
    ));
    category.add_question(Question::new(
        "What is 12 / 4?".to_string(),
        vec!["2".to_string(), "3".to_string(), "4".to_string()],
        1,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the decimal number 42?".to_string(),
        vec![
            "101010".to_string(),
            "110010".to_string(),
            "100110".to_string(),
            "101001".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal value of the hexadecimal number 1A?".to_string(),
        vec!["26".to_string(), "24".to_string(), "28".to_string(), "22".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 2^10?".to_string(),
        vec!["1024".to_string(), "512".to_string(), "2048".to_string(), "1000".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of the binary number 1111 1111?".to_string(),
        vec!["FF".to_string(), "F0".to_string(), "F1".to_string(), "FE".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the hexadecimal number C8?".to_string(),
        vec![
            "11001000".to_string(),
            "11011000".to_string(),
            "11101000".to_string(),
            "11110000".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the largest value that can be represented with 8 bits?".to_string(),
        vec!["255".to_string(), "256".to_string(), "254".to_string(), "512".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of shifting the binary number 0001 0010 to the left by 2 bits?".to_string(),
        vec![
            "0100 1000".to_string(),
            "0001 1000".to_string(),
            "0100 0000".to_string(),
            "0010 0100".to_string(),
        ],
        3,
    ));

    category.add_question(Question::new(
        "What is the decimal value of the binary number 101010?".to_string(),
        vec!["42".to_string(), "32".to_string(), "52".to_string(), "62".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many bits are in a byte?".to_string(),
        vec!["8".to_string(), "16".to_string(), "4".to_string(), "10".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 2^16?".to_string(),
        vec!["65536".to_string(), "32768".to_string(), "131072".to_string(), "262144".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the smallest positive 2's complement number representable with 8 bits?".to_string(),
        vec![
            "-128".to_string(),
            "-127".to_string(),
            "-256".to_string(),
            "0".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of the decimal number 255?".to_string(),
        vec!["FF".to_string(), "F0".to_string(), "FE".to_string(), "100".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary AND of 1100 and 1010?".to_string(),
        vec![
            "1000".to_string(),
            "1100".to_string(),
            "1110".to_string(),
            "1010".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "How many unique values can be represented with 10 bits?".to_string(),
        vec!["1024".to_string(), "512".to_string(), "2048".to_string(), "1000".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal equivalent of binary 1010 1010?".to_string(),
        vec!["AA".to_string(), "A8".to_string(), "AB".to_string(), "B0".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "Which of the following is a valid 8-bit binary number?".to_string(),
        vec![
            "11111111".to_string(),
            "100000000".to_string(),
            "1111".to_string(),
            "1".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the decimal number 17?".to_string(),
        vec![
            "10001".to_string(),
            "10111".to_string(),
            "11001".to_string(),
            "10011".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "How many bits are needed to represent the decimal number 500 in binary?".to_string(),
        vec!["9".to_string(), "10".to_string(), "8".to_string(), "11".to_string()],
        3,
    ));

    category.add_question(Question::new(
        "What is the result of flipping all bits in the binary number 10101010?".to_string(),
        vec![
            "01010101".to_string(),
            "11011011".to_string(),
            "00101010".to_string(),
            "11111111".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of adding the binary numbers 1101 and 1011?".to_string(),
        vec![
            "11000".to_string(),
            "10110".to_string(),
            "10000".to_string(),
            "11101".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 2^8?".to_string(),
        vec!["256".to_string(), "128".to_string(), "512".to_string(), "64".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of the decimal number 64?".to_string(),
        vec!["40".to_string(), "44".to_string(), "64".to_string(), "41".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the decimal number 19?".to_string(),
        vec![
            "10011".to_string(),
            "11001".to_string(),
            "10110".to_string(),
            "11100".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the value of π (pi) rounded to 3 decimal places?".to_string(),
        vec![
            "3.141".to_string(),
            "3.142".to_string(),
            "3.140".to_string(),
            "3.143".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "How many bytes are in a kilobyte?".to_string(),
        vec!["1024".to_string(), "1000".to_string(), "512".to_string(), "2048".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 1111 AND 1010 in binary?".to_string(),
        vec![
            "1010".to_string(),
            "1110".to_string(),
            "1100".to_string(),
            "1000".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the square root of 144?".to_string(),
        vec!["12".to_string(), "14".to_string(), "10".to_string(), "16".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 2^16?".to_string(),
        vec!["65536".to_string(), "32768".to_string(), "131072".to_string(), "4096".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the hexadecimal number 7F?".to_string(),
        vec![
            "01111111".to_string(),
            "11111111".to_string(),
            "10000000".to_string(),
            "10101010".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal equivalent of binary 101010?".to_string(),
        vec!["42".to_string(), "45".to_string(), "40".to_string(), "50".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the area of a circle with radius 7 (π ≈ 3.14)?".to_string(),
        vec![
            "153.86".to_string(),
            "150.00".to_string(),
            "140.12".to_string(),
            "160.00".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal value of 255 in decimal?".to_string(),
        vec!["FF".to_string(), "F0".to_string(), "FE".to_string(), "0F".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 12 * 12?".to_string(),
        vec!["144".to_string(), "132".to_string(), "156".to_string(), "150".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many unique values can 10 bits represent?".to_string(),
        vec!["1024".to_string(), "1000".to_string(), "2048".to_string(), "512".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary complement of 1010?".to_string(),
        vec!["0101".to_string(), "1001".to_string(), "1100".to_string(), "0110".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 3 * (4 + 5)?".to_string(),
        vec!["27".to_string(), "12".to_string(), "15".to_string(), "9".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of the binary number 1111 0000?".to_string(),
        vec!["F0".to_string(), "0F".to_string(), "FF".to_string(), "F1".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 256 / 16?".to_string(),
        vec!["16".to_string(), "8".to_string(), "32".to_string(), "12".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the sum of the first 10 natural numbers?".to_string(),
        vec!["55".to_string(), "50".to_string(), "60".to_string(), "45".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the 2's complement of 0110 (4 bits)?".to_string(),
        vec![
            "1010".to_string(),
            "1110".to_string(),
            "0100".to_string(),
            "1100".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 2^32?".to_string(),
        vec![
            "4,294,967,296".to_string(),
            "2,147,483,648".to_string(),
            "8,589,934,592".to_string(),
            "1,073,741,824".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal equivalent of hexadecimal 1A3?".to_string(),
        vec!["419".to_string(), "411".to_string(), "420".to_string(), "413".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many bits are in a byte?".to_string(),
        vec!["8".to_string(), "4".to_string(), "16".to_string(), "32".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary equivalent of the decimal number 85?".to_string(),
        vec![
            "1010101".to_string(),
            "1101101".to_string(),
            "1110111".to_string(),
            "1001101".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the sum of the angles in a triangle?".to_string(),
        vec![
            "180 degrees".to_string(),
            "90 degrees".to_string(),
            "360 degrees".to_string(),
            "270 degrees".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of hexadecimal F multiplied by 2?".to_string(),
        vec!["1E".to_string(), "2F".to_string(), "FF".to_string(), "10".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 64 divided by 2^3?".to_string(),
        vec!["8".to_string(), "16".to_string(), "4".to_string(), "2".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the value of log2(1024)?".to_string(),
        vec!["10".to_string(), "9".to_string(), "11".to_string(), "12".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal equivalent of binary 11010101?".to_string(),
        vec!["D5".to_string(), "C5".to_string(), "F5".to_string(), "A5".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the greatest common divisor (GCD) of 24 and 36?".to_string(),
        vec!["12".to_string(), "6".to_string(), "18".to_string(), "24".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary result of 1100 XOR 1010?".to_string(),
        vec![
            "0110".to_string(),
            "1110".to_string(),
            "1011".to_string(),
            "1101".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the square of 15?".to_string(),
        vec!["225".to_string(), "200".to_string(), "250".to_string(), "240".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal equivalent of binary 11111111?".to_string(),
        vec!["255".to_string(), "256".to_string(), "128".to_string(), "127".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 5 + 3 * 2?".to_string(),
        vec!["11".to_string(), "16".to_string(), "13".to_string(), "10".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the largest 8-bit unsigned integer value?".to_string(),
        vec![
            "255".to_string(),
            "256".to_string(),
            "127".to_string(),
            "128".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "How many hexadecimal digits are needed to represent 16 bits?".to_string(),
        vec!["4".to_string(), "8".to_string(), "16".to_string(), "2".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of binary 10101 OR 1100?".to_string(),
        vec![
            "11101".to_string(),
            "10101".to_string(),
            "11001".to_string(),
            "11100".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 7 factorial (7!)?".to_string(),
        vec![
            "5040".to_string(),
            "4032".to_string(),
            "2520".to_string(),
            "720".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal value of 4095 in decimal?".to_string(),
        vec!["FFF".to_string(), "FFE".to_string(), "FAF".to_string(), "FFF0".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the least common multiple (LCM) of 12 and 18?".to_string(),
        vec!["36".to_string(), "72".to_string(), "24".to_string(), "48".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many unique values can be represented with 16 bits?".to_string(),
        vec!["65536".to_string(), "32768".to_string(), "131072".to_string(), "1024".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary result of 1000 AND 1100?".to_string(),
        vec![
            "1000".to_string(),
            "1100".to_string(),
            "1010".to_string(),
            "1110".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 3 to the power of 4 (3^4)?".to_string(),
        vec!["81".to_string(), "27".to_string(), "64".to_string(), "72".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary equivalent of decimal 100?".to_string(),
        vec![
            "1100100".to_string(),
            "1001100".to_string(),
            "1010100".to_string(),
            "1110000".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of (7 + 3) * (8 - 5)?".to_string(),
        vec!["30".to_string(), "50".to_string(), "40".to_string(), "20".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of binary 11101111?".to_string(),
        vec!["EF".to_string(), "FE".to_string(), "FF".to_string(), "EE".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 100 / 2^3?".to_string(),
        vec!["12.5".to_string(), "25".to_string(), "10".to_string(), "20".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 256 / 2^4?".to_string(),
        vec!["16".to_string(), "8".to_string(), "32".to_string(), "12".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the hexadecimal number 3C?".to_string(),
        vec![
            "00111100".to_string(),
            "11110000".to_string(),
            "10101100".to_string(),
            "11101100".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 2^32?".to_string(),
        vec![
            "4,294,967,296".to_string(),
            "2,147,483,648".to_string(),
            "8,589,934,592".to_string(),
            "1,073,741,824".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal equivalent of hexadecimal 1A3?".to_string(),
        vec!["419".to_string(), "411".to_string(), "420".to_string(), "413".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many bits are in a byte?".to_string(),
        vec!["8".to_string(), "4".to_string(), "16".to_string(), "32".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary equivalent of the decimal number 85?".to_string(),
        vec![
            "1010101".to_string(),
            "1101101".to_string(),
            "1110111".to_string(),
            "1001101".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the sum of the angles in a triangle?".to_string(),
        vec![
            "180 degrees".to_string(),
            "90 degrees".to_string(),
            "360 degrees".to_string(),
            "270 degrees".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of hexadecimal F multiplied by 2?".to_string(),
        vec!["1E".to_string(), "2F".to_string(), "FF".to_string(), "10".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 64 divided by 2^3?".to_string(),
        vec!["8".to_string(), "16".to_string(), "4".to_string(), "2".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the value of log2(1024)?".to_string(),
        vec!["10".to_string(), "9".to_string(), "11".to_string(), "12".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal equivalent of binary 11010101?".to_string(),
        vec!["D5".to_string(), "C5".to_string(), "F5".to_string(), "A5".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the greatest common divisor (GCD) of 24 and 36?".to_string(),
        vec!["12".to_string(), "6".to_string(), "18".to_string(), "24".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary result of 1100 XOR 1010?".to_string(),
        vec![
            "0110".to_string(),
            "1110".to_string(),
            "1011".to_string(),
            "1101".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the square of 15?".to_string(),
        vec!["225".to_string(), "200".to_string(), "250".to_string(), "240".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the decimal equivalent of binary 11111111?".to_string(),
        vec!["255".to_string(), "256".to_string(), "128".to_string(), "127".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 5 + 3 * 2?".to_string(),
        vec!["11".to_string(), "16".to_string(), "13".to_string(), "10".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the largest 8-bit unsigned integer value?".to_string(),
        vec![
            "255".to_string(),
            "256".to_string(),
            "127".to_string(),
            "128".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "How many hexadecimal digits are needed to represent 16 bits?".to_string(),
        vec!["4".to_string(), "8".to_string(), "16".to_string(), "2".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of binary 10101 OR 1100?".to_string(),
        vec![
            "11101".to_string(),
            "10101".to_string(),
            "11001".to_string(),
            "11100".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 7 factorial (7!)?".to_string(),
        vec![
            "5040".to_string(),
            "4032".to_string(),
            "2520".to_string(),
            "720".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal value of 4095 in decimal?".to_string(),
        vec!["FFF".to_string(), "FFE".to_string(), "FAF".to_string(), "FFF0".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the least common multiple (LCM) of 12 and 18?".to_string(),
        vec!["36".to_string(), "72".to_string(), "24".to_string(), "48".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "How many unique values can be represented with 16 bits?".to_string(),
        vec!["65536".to_string(), "32768".to_string(), "131072".to_string(), "1024".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary result of 1000 AND 1100?".to_string(),
        vec![
            "1000".to_string(),
            "1100".to_string(),
            "1010".to_string(),
            "1110".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is 3 to the power of 4 (3^4)?".to_string(),
        vec!["81".to_string(), "27".to_string(), "64".to_string(), "72".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary equivalent of decimal 100?".to_string(),
        vec![
            "1100100".to_string(),
            "1001100".to_string(),
            "1010100".to_string(),
            "1110000".to_string(),
        ],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of (7 + 3) * (8 - 5)?".to_string(),
        vec!["30".to_string(), "50".to_string(), "40".to_string(), "20".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the hexadecimal representation of binary 11101111?".to_string(),
        vec!["EF".to_string(), "FE".to_string(), "FF".to_string(), "EE".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the result of 100 / 2^3?".to_string(),
        vec!["12.5".to_string(), "25".to_string(), "10".to_string(), "20".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is 256 / 2^4?".to_string(),
        vec!["16".to_string(), "8".to_string(), "32".to_string(), "12".to_string()],
        0,
    ));

    category.add_question(Question::new(
        "What is the binary representation of the hexadecimal number 3C?".to_string(),
        vec![
            "00111100".to_string(),
            "11110000".to_string(),
            "10101100".to_string(),
            "11101100".to_string(),
        ],
        0,
    ));
    // Kopieren Sie hier einfach alle weiteren Mathematik-Fragen aus Ihrer data.rs,
    // der Aufruf bleibt category.add_question(Question::new(...))
}
