use crate::{make_term, Schema};
use pyo3::{exceptions, prelude::*, types::PyAny, types::PyList};
use tantivy as tv;

/// Tantivy's Query
#[pyclass(frozen, module = "tantivy.tantivy")]
pub(crate) struct Query {
    pub(crate) inner: Box<dyn tv::query::Query>,
}

impl Query {
    pub(crate) fn get(&self) -> &dyn tv::query::Query {
        &self.inner
    }
}

#[pymethods]
impl Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.get()))
    }

    /// Construct a Tantivy's TermQuery
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, field_value, index_option = "position"))]
    pub(crate) fn term_query(
        schema: &Schema,
        field_name: &str,
        field_value: &PyAny,
        index_option: &str,
    ) -> PyResult<Query> {
        let term = make_term(&schema.inner, field_name, field_value)?;
        let index_option = match index_option {
            "position" => tv::schema::IndexRecordOption::WithFreqsAndPositions,
            "freq" => tv::schema::IndexRecordOption::WithFreqs,
            "basic" => tv::schema::IndexRecordOption::Basic,
            _ => return Err(exceptions::PyValueError::new_err(
                "Invalid index option, valid choices are: 'basic', 'freq' and 'position'"
            ))
        };
        let inner = tv::query::TermQuery::new(term, index_option);
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    /// Construct a Tantivy's AllQuery
    #[staticmethod]
    pub(crate) fn all_query() -> PyResult<Query> {
        let inner = tv::query::AllQuery {};
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    /// Construct a Tantivy's PhraseQuery
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, words))]
    pub(crate) fn phrase_query(
        schema: &Schema,
        field_name: &str,
        words: &PyList,
    ) -> PyResult<Query> {
        let mut terms = Vec::<tv::Term>::new();
        for w in words.iter() {
            let term = make_term(&schema.inner, field_name, w)?;
            terms.push(term);
        }
        let inner = tv::query::PhraseQuery::new(terms);
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    /// Construct a Tantivy's PhraseQuery with custom offsets and slop
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, words, offsets, slop = 0))]
    pub(crate) fn phrase_query_offset_slop(
        schema: &Schema,
        field_name: &str,
        words: &PyList,
        offsets: &PyList,
        slop: u32,
    ) -> PyResult<Query> {
        assert!(
            words.len() == offsets.len(),
            "'words' and 'offsets' must be the same size."
        );
        let mut terms = Vec::<(usize, tv::Term)>::new();
        for (o, w) in offsets.iter().zip(words.iter()) {
            let offset = o.extract::<usize>()?;
            let term = make_term(&schema.inner, field_name, w)?;
            terms.push((offset, term));
        }
        let inner =
            tv::query::PhraseQuery::new_with_offset_and_slop(terms, slop);
        Ok(Query {
            inner: Box::new(inner),
        })
    }
}
