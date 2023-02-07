mod filter_constants;
pub mod outer_punctuation_filter;
pub mod possessive_contraction_filter;

use filter_constants::STOPWORDS_EN;

pub fn get_stopwords_filter_en() -> Vec<String> {
    let mut words = Vec::new();
    for word in STOPWORDS_EN {
        words.push(String::from(word))
    }
    words
}
