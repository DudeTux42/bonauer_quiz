use crate::category::Category;
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
        "What does CPU stand for?".to_string(),
        vec!["Central Process Unit".to_string(), "Central Processing Unit".to_string()],
        1,
    ));
    it_category.add_question(Question::new(
        "What does HTML stand for?".to_string(),
        vec!["Hyper Text Markup Language".to_string(), "Hyper Transfer Markup Language".to_string()],
        0,
    ));
    it_category.add_question(Question::new(
        "What is the primary language used for web development?".to_string(),
        vec!["Java".to_string(), "JavaScript".to_string(), "C++".to_string()],
        1,
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

    // category IPV4
    let mut ipv4_classes_category = Category::new("IPV4".to_string());

    ipv4_classes_category.add_question(Question::new(
        "Welche Klasse hat diese IPV4 Adresse: {}".to_string(),
        vec!["Klasse A".to_string(), "Klasse B".to_string(), "Klasse C".to_string(), "Klasse D".to_string(), "Klasse E".to_string()],
        0, //TODO: figure out how to handle correct answer
    ));




    // add categories to quiz
    quiz.add_category(math_category);
    quiz.add_category(it_category);
    quiz.add_category(abbreviations_category);

    quiz
}

