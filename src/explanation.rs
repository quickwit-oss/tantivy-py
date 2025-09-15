use pyo3::prelude::*;
use tantivy as tv;

/// Represents an explanation of how a document matched a query.
#[pyclass(frozen, module = "tantivy.tantivy")]
pub(crate) struct Explanation {
    inner: tv::query::Explanation,
}

impl Explanation {
    pub(crate) fn new(inner: tv::query::Explanation) -> Self {
        Explanation { inner }
    }
}

#[pymethods]
impl Explanation {
    /// Returns a JSON representation of the explanation.
    fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_pretty_json())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Explanation(value={})", self.inner.value()))
    }
}
