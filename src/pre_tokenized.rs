use pyo3::{exceptions::PyValueError, prelude::*, IntoPyObjectExt};
use tantivy::tokenizer as tvt;

#[derive(Clone)]
#[pyclass(module = "tantivy.tokenizer")]
pub(crate) struct Token {
    #[pyo3(get)]
    pub(crate) text: String,
    #[pyo3(get)]
    pub(crate) position: usize,
    #[pyo3(get)]
    pub(crate) character_offset_from: usize,
    #[pyo3(get)]
    pub(crate) character_offset_to: usize,
}

// Token used to create a PreTokenizedString.
#[pymethods]
impl Token {
    #[new]
    /// Creates a new Token.
    /// The offsets are unicode character offsets, like the one you get if you index a string in Python,
    /// and not byte offsets like the ones used by Tantivy's Token struct.
    ///
    /// Args:
    ///     text (str): Actual text content of the token.
    ///     position (int): The position, expressed in number of tokens.
    ///     character_offset_from (int): Offset (codepoint index) of the first character of the token.
    ///     character_offset_to (int, optional):
    ///         Offset (codepoint index) of the last character of the token + 1.
    ///         The text that generated the token should be obtained by text[character_offset_from:character_offset_to].
    ///         The offset is calculated from the length of the string if not provided.
    ///         Should be set if the token text does not have the same length as the original text that generated it, e.g. if the token is a stemmed version of the original text.
    #[pyo3(signature = (text, position, character_offset_from, character_offset_to=None))]
    fn new(
        text: &str,
        position: usize,
        character_offset_from: usize,
        character_offset_to: Option<usize>,
    ) -> Self {
        let character_offset_to = character_offset_to
            .unwrap_or_else(|| character_offset_from + text.chars().count());
        Self {
            text: text.to_owned(),
            position,
            character_offset_from,
            character_offset_to,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Token(text='{}', position={}, character_offset_from={}, character_offset_to={})",
            self.text, self.position, self.character_offset_from, self.character_offset_to
        )
    }

    fn __richcmp__(
        &self,
        other: &Self,
        op: pyo3::basic::CompareOp,
        py: Python<'_>,
    ) -> PyResult<Py<PyAny>> {
        match op {
            pyo3::basic::CompareOp::Eq => (self.text == other.text
                && self.position == other.position
                && self.character_offset_from == other.character_offset_from
                && self.character_offset_to == other.character_offset_to)
                .into_py_any(py),
            pyo3::basic::CompareOp::Ne => (self.text != other.text
                || self.position != other.position
                || self.character_offset_from != other.character_offset_from
                || self.character_offset_to != other.character_offset_to)
                .into_py_any(py),
            _ => Ok(py.NotImplemented()),
        }
    }
}

/// Tantivy's PreTokenizedString

#[pyclass(module = "tantivy.tokenizer")]
pub(crate) struct PreTokenizedString {
    pub(crate) inner: tvt::PreTokenizedString,
}

#[pymethods]
impl PreTokenizedString {
    #[new]
    /// Creates a new PreTokenizedString.
    /// Args:
    ///     text (str): The original text.
    ///     tokens (Sequence[Token]): Tokens derived from the text.
    fn new(text: &str, tokens: Vec<Token>) -> PyResult<Self> {
        let byte_offsets: Vec<_> =
            text.char_indices().map(|(offset, _)| offset).collect();

        Ok(Self {
            inner: tvt::PreTokenizedString {
                text: text.to_string(),
                tokens: tokens
                    .into_iter()
                    .map(|token| {
                        let offset_from = byte_offsets
                            .get(token.character_offset_from)
                            .copied()
                            .unwrap_or(text.len());
                        let offset_to = byte_offsets
                            .get(token.character_offset_to)
                            .copied()
                            .unwrap_or(text.len());
                        tvt::Token {
                            offset_from,
                            offset_to,
                            position: token.position,
                            text: token.text.clone(),
                            position_length: 1,
                        }
                    })
                    .collect(),
            },
        })
    }
    #[getter]
    fn text(&self) -> String {
        self.inner.text.clone()
    }
    #[getter]
    fn tokens(&self) -> Vec<Token> {
        let byte_offsets: Vec<_> = self
            .inner
            .text
            .char_indices()
            .map(|(offset, _)| offset)
            .collect();
        let character_count = byte_offsets.len();

        self.inner
            .tokens
            .iter()
            .map(|token| {
                let character_offset_from = byte_offsets
                    .binary_search(&token.offset_from)
                    .unwrap_or(character_count);
                let character_offset_to = byte_offsets
                    .binary_search(&token.offset_to)
                    .unwrap_or(character_count);

                Token {
                    text: token.text.clone(),
                    position: token.position,
                    character_offset_from,
                    character_offset_to,
                }
            })
            .collect()
    }
}
