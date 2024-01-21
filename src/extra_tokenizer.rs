// Define a config block that only activates when the "lindera" feature is enabled.
// This is used to add the Lindera tokenizer to the default tokenizer chain.
// See
// https://docs.rs/tantivy/0.16.0/tantivy/struct.Index.html#method.LinderaTokenizer
// for more information.

#[cfg(feature = "lindera")]
use lindera_core::mode::Mode;
use lindera_dictionary::{load_dictionary_from_config, DictionaryConfig, DictionaryKind};
use lindera_tantivy::tokenizer::LinderaTokenizer;

pub fn register_lindera(mut index: tantivy::Index) {
    // Tokenizer with IPADIC
    let dictionary_config = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let dictionary = load_dictionary(dictionary_config).unwrap();
    let tokenizer = LinderaTokenizer::new(dictionary, None, Mode::Normal);

    // register Lindera tokenizer
    index.tokenizers().register("lang_ja", tokenizer);
}
