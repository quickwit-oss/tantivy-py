use lindera_core::mode::Mode;
use lindera_dictionary::{
    load_dictionary_from_config, DictionaryConfig, DictionaryKind,
};
use lindera_tantivy::tokenizer::LinderaTokenizer;
use pyo3::{pyclass, pymethods, FromPyObject};

#[pyclass]
#[derive(Clone)]
pub enum LinderaDictionaryKind {
    IPADIC,
    IPADICNEologd,
    UniDic,
    KoDic,
    CcCedict,
}

impl From<LinderaDictionaryKind> for DictionaryKind {
    fn from(kind: LinderaDictionaryKind) -> Self {
        match kind {
            LinderaDictionaryKind::IPADIC => DictionaryKind::IPADIC,
            LinderaDictionaryKind::IPADICNEologd => {
                DictionaryKind::IPADICNEologd
            }
            LinderaDictionaryKind::UniDic => DictionaryKind::UniDic,
            LinderaDictionaryKind::KoDic => DictionaryKind::KoDic,
            LinderaDictionaryKind::CcCedict => DictionaryKind::CcCedict,
        }
    }
}

#[pyclass(get_all, set_all)]
#[derive(Clone)]
pub struct LinderaModeDecomposePenalty {
    kanji_penalty_length_threshold: usize,
    kanji_penalty_length_penalty: i32,
    other_penalty_length_threshold: usize,
    other_penalty_length_penalty: i32,
}

impl Default for LinderaModeDecomposePenalty {
    fn default() -> Self {
        LinderaModeDecomposePenalty {
            kanji_penalty_length_threshold: 2,
            kanji_penalty_length_penalty: 3000,
            other_penalty_length_threshold: 7,
            other_penalty_length_penalty: 1700,
        }
    }
}

#[pymethods]
impl LinderaModeDecomposePenalty {
    #[new]
    #[pyo3(signature = (
    kanji_penalty_length_threshold = 2,
    kanji_penalty_length_penalty = 3000,
    other_penalty_length_threshold = 7,
    other_penalty_length_penalty = 1700
    ))]
    pub fn new(
        kanji_penalty_length_threshold: usize,
        kanji_penalty_length_penalty: i32,
        other_penalty_length_threshold: usize,
        other_penalty_length_penalty: i32,
    ) -> Self {
        LinderaModeDecomposePenalty {
            kanji_penalty_length_threshold,
            kanji_penalty_length_penalty,
            other_penalty_length_threshold,
            other_penalty_length_penalty,
        }
    }
}

impl From<LinderaModeDecomposePenalty> for lindera_core::mode::Penalty {
    fn from(penalty: LinderaModeDecomposePenalty) -> Self {
        lindera_core::mode::Penalty {
            kanji_penalty_length_threshold: penalty
                .kanji_penalty_length_threshold,
            kanji_penalty_length_penalty: penalty.kanji_penalty_length_penalty,
            other_penalty_length_threshold: penalty
                .other_penalty_length_threshold,
            other_penalty_length_penalty: penalty.other_penalty_length_penalty,
        }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
pub struct LNormal {}

impl From<LNormal> for Mode {
    fn from(_: LNormal) -> Self {
        Mode::Normal
    }
}

#[pymethods]
impl LNormal {
    #[new]
    pub fn new() -> Self {
        LNormal {}
    }
}

#[pyclass(frozen, get_all)]
#[derive(Clone)]
pub struct LDecompose {
    penalty: LinderaModeDecomposePenalty,
}

impl From<LDecompose> for Mode {
    fn from(decompose: LDecompose) -> Self {
        Mode::Decompose(decompose.penalty.into())
    }
}

#[pymethods]
impl LDecompose {
    #[new]
    pub fn new(penalty: Option<LinderaModeDecomposePenalty>) -> Self {
        LDecompose {
            penalty: penalty.unwrap_or_default(),
        }
    }
}

pub fn create_tokenizer(
    mode: Mode,
    dictionary_kind: DictionaryKind,
) -> LinderaTokenizer {
    let dictionary_config = DictionaryConfig {
        kind: Some(dictionary_kind.into()),
        path: None,
    };
    let dictionary = load_dictionary_from_config(dictionary_config).unwrap();
    let tokenizer = LinderaTokenizer::new(dictionary, None, mode);

    tokenizer
}
