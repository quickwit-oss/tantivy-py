use lindera_core::mode::Mode;
use lindera_dictionary::{
    load_dictionary_from_config, DictionaryConfig, DictionaryKind,
};
use lindera_tantivy::tokenizer::LinderaTokenizer;

pub fn create_tokenizer(mode: Mode) -> LinderaTokenizer {
    let dictionary_config = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let dictionary = load_dictionary_from_config(dictionary_config).unwrap();
    let tokenizer = LinderaTokenizer::new(dictionary, None, mode);

    tokenizer
}
