use std::mem;

use tantivy::tokenizer::Tokenizer;
use tantivy::tokenizer::{Token, TokenFilter, TokenStream};
use unicode_properties::GeneralCategoryGroup;
use unicode_properties::UnicodeGeneralCategory;

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

fn is_disallowed_category(c: &char) -> bool {
    let category = c.general_category_group();
    // The only categories allowed through are:
    // GeneralCategoryGroup::Letter
    // GeneralCategoryGroup::Number
    // GeneralCategoryGroup::Symbol (includes emoji)

    match category {
        GeneralCategoryGroup::Mark => true,
        GeneralCategoryGroup::Punctuation => true,
        GeneralCategoryGroup::Separator => true,
        GeneralCategoryGroup::Other => true,
        _ => false,
    }
}

pub struct OuterPunctuationFilterTokenStream<T> {
    leading_allow: Vec<char>,
    // buffer acts as temporary string memory to switch out token text.
    buffer: String,
    tail: T,
}

// Trims the token stream of any leading/ trailing punctuations.
impl<T: TokenStream> TokenStream for OuterPunctuationFilterTokenStream<T> {
    fn advance(&mut self) -> bool {
        while self.tail.advance() {
            let token_text = &self.tail.token().text;

            // Strip leading punctuation
            let token_text = token_text.trim_start_matches(|c: char| {
                (c.is_ascii_punctuation() || is_disallowed_category(&c))
                    && !self.leading_allow.contains(&c)
            });

            // Strip trailing punctuation
            let token_text = token_text.trim_end_matches(|c: char| {
                c.is_ascii_punctuation() || is_disallowed_category(&c)
            });

            self.buffer.clear();
            self.buffer.push_str(token_text);
            // Replace the token text with the trimmed word
            mem::swap(&mut self.tail.token_mut().text, &mut self.buffer);
            return true;
        }
        false
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
        println!("tokens {:?}", tokens);
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

    #[test]
    fn test_to_outer_punctuation_filter_emoji() {
        let tokens = token_stream_helper("🌳");
        println!("emoji tokens {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "🌳", 0, 4);
    }

    #[test]
    fn test_to_outer_punctuation_filter_emoji2() {
        let tokens = token_stream_helper("tree🌳");
        println!("emoji tokens {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], 0, "tree🌳", 0, 8);
    }

    #[test]
    fn test_to_outer_punctuation_filter_emoji3() {
        let tokens = token_stream_helper("tree ?🌳");
        println!("emoji tokens {:?}", tokens);
        assert_eq!(tokens.len(), 2);
        assert_token(&tokens[0], 0, "tree", 0, 4);
        assert_token(&tokens[1], 1, "🌳", 5, 10);
    }

    #[test]
    fn test_to_outer_punctuation_filter_emoji4() {
        let cases = vec![
            // Plain text
            ("tree", "tree"),
            ("tree tree", "tree tree"),
            ("tree tree tree", "tree tree tree"),
            // Emoji only
            ("🌳", "🌳"),
            // Unicode flag test
            ("🇦🇺", "🇦🇺"),
            ("🇦🇺🌳", "🇦🇺🌳"),
            ("🌳🇦🇺", "🌳🇦🇺"),
            // Mixed text and emoji
            ("tree🌳", "tree🌳"),
            ("tree 🌳", "tree 🌳"),
            ("tree🌳 tree", "tree🌳 tree"),
            ("tree tree🌳", "tree tree🌳"),
            // ASCII Punctuation
            ("???🌳???", "🌳"),
            ("...🌳...", "🌳"),
            ("//tree ?🌳//", "tree 🌳"),
            // Some punctuation must be let through
            ("#tree🌳", "#tree🌳"),
            ("@tree🌳", "@tree🌳"),
            // But only if it is at the start. At the end still drops.
            ("tree🌳#", "tree🌳"),
            ("tree🌳@", "tree🌳"),
            // Unicode Punctuation with odd characters and quotes
            ("-tree 🌳", "tree 🌳"),
            ("—tree 🌳", "tree 🌳"),
            ("⸗tree 🌳", "tree 🌳"),
            ("⸚tree 🌳", "tree 🌳"),
            ("⸺tree 🌳", "tree 🌳"),
            ("〜tree 🌳", "tree 🌳"),
            ("〰tree 🌳", "tree 🌳"),
            ("«tree 🌳»", "tree 🌳"),
            ("‘tree 🌳", "tree 🌳"),
            ("“tree 🌳", "tree 🌳"),
            ("⸄tree 🌳", "tree 🌳"),
            ("⸉tree 🌳", "tree 🌳"),
            ("❨tree 🌳", "tree 🌳"),
            ("⸦tree 🌳", "tree 🌳"),
            ("『tree 🌳』", "tree 🌳"),
            ("¿tree 🌳", "tree 🌳"),
            // Greek question mark NOT semicolon
            (";tree 🌳", "tree 🌳"),
            ("‡tree 🌳", "tree 🌳"),
            ("‥tree 🌳", "tree 🌳"),
            ("‴tree 🌳", "tree 🌳"),
            ("※tree 🌳", "tree 🌳"),
            ("⁂tree 🌳", "tree 🌳"),
            ("⁜tree 🌳", "tree 🌳"),
            ("﹅tree 🌳", "tree 🌳"),
        ];
        for (input, expected) in cases {
            let out = token_full_pipeline(input);
            println!("out {:?}", out);
            assert_eq!(out, expected);
        }
    }

    fn token_full_pipeline(text: &str) -> String {
        let tokens = token_stream_helper(text);
        println!("emoji tokens {:?}", tokens);
        let token_string = tokens
            .iter()
            .map(|token| token.text.clone())
            .collect::<Vec<String>>()
            .join(" ");
        token_string
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
