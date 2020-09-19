use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use std::sync::Arc;
use tantivy as tv;

/// Tantivy's Query
#[pyclass]
pub(crate) struct Query {
    pub(crate) query: Arc<String>,
    pub(crate) parser: tv::query::QueryParser,
}

impl Query {
    pub(crate) fn get(&self) -> Box<dyn tv::query::Query> {
        self.parser
            .parse_query(&self.query)
            .expect("Created a query that returns a parse error")
    }
}

#[pyproto]
impl PyObjectProtocol for Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.get()))
    }
}
