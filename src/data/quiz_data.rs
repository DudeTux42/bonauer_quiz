use crate::models::{Category, Quiz};
use crate::utils::ipv4::generate_ipv4_question;
use crate::data::questions::{math, it, abbreviations};

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

    quiz
}
