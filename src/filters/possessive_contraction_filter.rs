use std::mem;

use crate::filters::filter_constants::CONTRACTION_PATTERNS;
use tantivy::tokenizer::BoxTokenStream;
use tantivy::tokenizer::{Token, TokenFilter, TokenStream};

//    Removes possessive contractions from tokens.
//    Is fairly robust in that is uses all know unicode apostrophe characters except U+02EE. See
//    `https://en.wikipedia.org/wiki/Apostrophe#Unicode`_.

#[derive(Clone)]
pub struct PossessiveContractionFilter;

impl TokenFilter for PossessiveContractionFilter {
    fn transform<'a>(
        &self,
        token_stream: BoxTokenStream<'a>,
    ) -> BoxTokenStream<'a> {
        BoxTokenStream::from(PossessiveContractionFilterTokenStream {
            tail: token_stream,
            buffer: String::with_capacity(100),
        })
    }
}

pub struct PossessiveContractionFilterTokenStream<'a> {
    // buffer acts as temporary string memory to switch out token text.
    buffer: String,
    tail: BoxTokenStream<'a>,
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
    return replaced;
}

impl<'a> TokenStream for PossessiveContractionFilterTokenStream<'a> {
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
        let mut token_stream = TextAnalyzer::from(WhitespaceTokenizer)
            .filter(PossessiveContractionFilter)
            .token_stream(text);
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
