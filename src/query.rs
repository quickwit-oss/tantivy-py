use pyo3::prelude::*;
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

#[pymethods]
impl Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.get()))
    }

    pub fn __copy__(&self) -> Self {
        Self {
            inner: self.get().box_clone(),
        }
    }

    pub fn __deepcopy__(&self) -> Self {
        Self {
            inner: self.get().box_clone(),
        }
    }
}
