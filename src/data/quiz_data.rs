use crate::data::questions::{abbreviations, it, math};
use crate::models::category::Category;
use crate::models::quiz::Quiz;
use crate::utils::ipv4::{generate_ipv4_question, generate_subnetting_question};

pub fn create_sample_quiz() -> Quiz {
    let mut quiz = Quiz::new();

    // Mathematik-Kategorie
    let mut math_category = Category::new("Mathematics".to_string());
    math::add_math_questions(&mut math_category);
    quiz.add_category(math_category);

    // IT-Kategorie
    let mut it_category = Category::new("IT".to_string());
    it::add_it_questions(&mut it_category);
    quiz.add_category(it_category);

    // Abkürzungs-Kategorie
    let mut abbreviations_category = Category::new("Abbreviations".to_string());
    abbreviations::add_abbreviation_questions(&mut abbreviations_category);
    quiz.add_category(abbreviations_category);

    // IPv4-Kategorie
    let mut ipv4_category = Category::new("IPV4".to_string());
    for _ in 0..10 {
        ipv4_category.add_question(generate_ipv4_question());
    }
    quiz.add_category(ipv4_category);

    //Subnetting-Kategorie
    let mut subnetting_category = Category::new("Subnetting".to_string());
    for _ in 0..10 {
        subnetting_category.add_question(generate_subnetting_question());
    }
    quiz.add_category(subnetting_category);

    quiz
}

/// Get a list of all category names from a quiz
pub fn get_categories(quiz: &Quiz) -> Vec<String> {
    quiz.categories().into_iter().map(String::from).collect()
}

/// Initialize questions for a specific category
pub fn get_category_questions(
    quiz: &Quiz,
    category_name: &str,
) -> Vec<crate::models::question::Question> {
    quiz.initialize_questions(category_name)
}
