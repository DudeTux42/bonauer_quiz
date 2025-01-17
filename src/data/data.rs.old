use crate::category::Category;
use crate::ipv4::generate_ipv4_question;
use crate::question::Question;
use crate::quiz::Quiz;

pub fn create_sample_quiz() -> Quiz {
    let mut quiz = Quiz::new();

    // Category: Math
    let mut math_category = Category::new("Mathematics".to_string());


    // Category: IT
    let mut it_category = Category::new("IT".to_string());
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

