use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use tantivy as tv;

/// Tantivy's Query
#[pyclass]
pub(crate) struct Query {
    pub(crate) inner: Box<dyn tv::query::Query>,
}

#[pyproto]
impl PyObjectProtocol for Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.inner))
    }
}
