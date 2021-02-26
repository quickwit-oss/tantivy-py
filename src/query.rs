use pyo3::{prelude::*, PyObjectProtocol};
use tantivy as tv;

/// Tantivy's Query
#[pyclass]
pub(crate) struct Query {
    pub(crate) inner: Box<dyn tv::query::Query>,
}

impl Query {
    pub(crate) fn get(&self) -> &dyn tv::query::Query {
        &self.inner
    }
}

#[pyproto]
impl PyObjectProtocol for Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.get()))
    }
}
