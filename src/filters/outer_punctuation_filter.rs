use std::mem;

use tantivy::tokenizer::{BoxTokenStream, Tokenizer};
use tantivy::tokenizer::{Token, TokenFilter, TokenStream};

// 'OuterPunctuationFilter' removes any leading or trailing punctuations from tokens.
// An array of punctuation characters (leading_allow) can be provided
// to exclude from this filtering process for leading punctuation.

#[derive(Clone)]
pub struct OuterPunctuationFilter {
    leading_allow: Vec<char>,
}

impl TokenFilter for OuterPunctuationFilter {
    type Tokenizer<T: Tokenizer> = OuterPunctuationFilterWrapper<T>;

    fn transform<T: Tokenizer>(
        self,
        tokenizer: T,
    ) -> OuterPunctuationFilterWrapper<T> {
        OuterPunctuationFilterWrapper {
            leading_allow: self.leading_allow,
            inner: tokenizer,
        }
    }
}

impl OuterPunctuationFilter {
    /// Creates a `OuterPunctuationFilter` given an array of exception punctuations.
    pub fn new(leading_allow: Vec<char>) -> OuterPunctuationFilter {
        OuterPunctuationFilter { leading_allow }
    }
}

#[derive(Clone)]
pub struct OuterPunctuationFilterWrapper<T> {
    leading_allow: Vec<char>,
    inner: T,
}

impl<T: Tokenizer> Tokenizer for OuterPunctuationFilterWrapper<T> {
    type TokenStream<'a> =
        OuterPunctuationFilterTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        OuterPunctuationFilterTokenStream {
            leading_allow: self.leading_allow.clone(),
            buffer: String::with_capacity(100),
            tail: self.inner.token_stream(text),
        }
    }
}

pub struct OuterPunctuationFilterTokenStream<T> {
    leading_allow: Vec<char>,
    // buffer acts as temporary string memory to switch out token text.
    buffer: String,
    tail: T,
}

pub fn trim_end(text: &str, output: &mut String) {
    output.clear();
    output.push_str(text.trim_end_matches(|c: char| !c.is_alphanumeric()));
}

pub fn trim_start(leading_allow: &Vec<char>, text: &str, output: &mut String) {
    output.clear();
    output.push_str(text.trim_start_matches(|c: char| {
        !c.is_alphanumeric() && !leading_allow.contains(&c)
    }));
}

// Trims the token stream of any leading/ trailing punctuations.
impl<T: TokenStream> TokenStream for OuterPunctuationFilterTokenStream<T> {
    fn advance(&mut self) -> bool {
        // stop if tail is empty
        if !self.tail.advance() {
            return false;
        }
        // trim the end of token text
        trim_end(&self.tail.token().text, &mut self.buffer);
        mem::swap(&mut self.tail.token_mut().text, &mut self.buffer);

        // trim start of the token text
        trim_start(
            &self.leading_allow,
            &self.tail.token().text,
            &mut self.buffer,
        );
        mem::swap(&mut self.tail.token_mut().text, &mut self.buffer);
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
pub mod tests {
    use super::OuterPunctuationFilter;
    use tantivy::tokenizer::{TextAnalyzer, Token, WhitespaceTokenizer};

    /// This is a function that can be used in tests and doc tests
    /// to assert a token's correctness.
    pub fn assert_token(
        token: &Token,
        position: usize,
        text: &str,
        from: usize,
        to: usize,
    ) {
        assert_eq!(
            token.position, position,
            "expected position {} but {:?}",
            position, token
        );
        assert_eq!(token.text, text, "expected text {} but {:?}", text, token);
        assert_eq!(
            token.offset_from, from,
            "expected offset_from {} but {:?}",
            from, token
        );
        assert_eq!(
            token.offset_to, to,
            "expected offset_to {} but {:?}",
            to, token
        );
    }

    #[test]
    fn test_to_outer_punctuation_filter() {
        let tokens = token_stream_helper("Tree**%^");
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "Tree", 0, 8);

        let tokens = token_stream_helper("To be or NOT%% to bee...");
        assert_eq!(tokens.len(), 6);
        assert_token(&tokens[0], 0, "To", 0, 2);
        assert_token(&tokens[1], 1, "be", 3, 5);
        assert_token(&tokens[2], 2, "or", 6, 8);
        assert_token(&tokens[3], 3, "NOT", 9, 14);
        assert_token(&tokens[4], 4, "to", 15, 17);
        assert_token(&tokens[5], 5, "bee", 18, 24);

        let tokens = token_stream_helper("@#Tree**%^");
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "@#Tree", 0, 10);
    }

    fn token_stream_helper(text: &str) -> Vec<Token> {
        let mut analyzer =
            TextAnalyzer::builder(WhitespaceTokenizer::default())
                .filter(OuterPunctuationFilter::new(vec!['#', '@']))
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
