use jieba_rs::Jieba;
use std::sync::Arc;

use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

pub const JIEBA: &str = "jieba";

#[derive(Debug)]
pub struct JiebaTokenStream<'a> {
    src: &'a str,
    result: Vec<&'a str>,
    index: usize,
    token: Token,
}

impl<'a> JiebaTokenStream<'a> {
    pub fn new(src: &'a str, result: Vec<&'a str>) -> Self {
        JiebaTokenStream {
            src,
            result,
            index: 0,
            token: Token::default(),
        }
    }
}

impl<'a> TokenStream for JiebaTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if self.index < self.result.len() {
            let current_word = self.result[self.index];
            let offset_from =
                current_word.as_ptr() as usize - self.src.as_ptr() as usize;
            let offset_to = offset_from + current_word.len();

            self.token = Token {
                offset_from,
                offset_to,
                position: self.index,
                text: current_word.to_string(),
                position_length: 1,
            };

            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}

#[derive(Clone, Debug)]
pub struct JiebaTokenizer {
    pub worker: Arc<Jieba>,
}

impl Default for JiebaTokenizer {
    fn default() -> Self {
        JiebaTokenizer {
            worker: Arc::new(Jieba::new()),
        }
    }
}

impl Tokenizer for JiebaTokenizer {
    type TokenStream<'a> = JiebaTokenStream<'a>;

    fn token_stream<'a>(&mut self, text: &'a str) -> JiebaTokenStream<'a> {
        let result = self.worker.cut_for_search(text, true);
        JiebaTokenStream::new(text, result)
    }
}
