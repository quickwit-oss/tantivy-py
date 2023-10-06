use std::mem;

use crate::filters::filter_constants::CONTRACTION_PATTERNS;
use tantivy::tokenizer::Tokenizer;
use tantivy::tokenizer::{Token, TokenFilter, TokenStream};

//    Removes possessive contractions from tokens.
//    Is fairly robust in that is uses all know unicode apostrophe characters except U+02EE. See
//    `https://en.wikipedia.org/wiki/Apostrophe#Unicode`_.

#[derive(Clone)]
pub struct PossessiveContractionFilter;

impl TokenFilter for PossessiveContractionFilter {
    type Tokenizer<T: Tokenizer> = PossessiveContractionFilterWrapper<T>;

    fn transform<T: Tokenizer>(
        self,
        tokenizer: T,
    ) -> PossessiveContractionFilterWrapper<T> {
        PossessiveContractionFilterWrapper { inner: tokenizer }
    }
}

#[derive(Clone)]
pub struct PossessiveContractionFilterWrapper<T> {
    inner: T,
}

impl<T: Tokenizer> Tokenizer for PossessiveContractionFilterWrapper<T> {
    type TokenStream<'a> =
        PossessiveContractionFilterTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        PossessiveContractionFilterTokenStream {
            buffer: String::with_capacity(100),
            tail: self.inner.token_stream(text),
        }
    }
}

pub struct PossessiveContractionFilterTokenStream<T> {
    // buffer acts as temporary string memory to switch out token text.
    buffer: String,
    tail: T,
}

// Creates desired string with possessive contractions substituted in the output string.
// Returns Tru if replacements were made, false otherwise.
pub fn replace_possessive_contractions(
    text: &str,
    output: &mut String,
) -> bool {
    output.clear();
    let mut replaced = false;
    let mut temp = String::from(text);
    for pat in CONTRACTION_PATTERNS {
        if temp.contains(pat) {
            temp = temp.replace(pat, "");
            replaced = true
        }
    }
    if replaced {
        output.push_str(&temp);
    }
    replaced
}

impl<T: TokenStream> TokenStream for PossessiveContractionFilterTokenStream<T> {
    fn advance(&mut self) -> bool {
        // stop if tail is empty
        if !self.tail.advance() {
            return false;
        }
        // replace possessive contractions if there are substritutions
        if replace_possessive_contractions(
            &self.tail.token().text,
            &mut self.buffer,
        ) {
            mem::swap(&mut self.tail.token_mut().text, &mut self.buffer);
        }
        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::filters::outer_punctuation_filter::tests::assert_token;
    use tantivy::tokenizer::{TextAnalyzer, Token, WhitespaceTokenizer};

    use super::PossessiveContractionFilter;

    #[test]
    fn test_to_outer_punctuation_filter() {
        let tokens = token_stream_helper("goku's");
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "goku", 0, 6);

        let tokens =
            token_stream_helper("your\u{2019}s mcdonald\u{02BC}s bee's");
        assert_eq!(tokens.len(), 3);

        assert_token(&tokens[0], 0, "your", 0, 8);
        assert_token(&tokens[1], 1, "mcdonald", 9, 20);
        assert_token(&tokens[2], 2, "bee", 21, 26);

        let tokens = token_stream_helper("Tree\u{A78B}s");
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "Tree", 0, 8);
    }

    fn token_stream_helper(text: &str) -> Vec<Token> {
        let mut analyzer =
            TextAnalyzer::builder(WhitespaceTokenizer::default())
                .filter(PossessiveContractionFilter)
                .build();
        let mut token_stream = analyzer.token_stream(text);
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
