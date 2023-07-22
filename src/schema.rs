use pyo3::{basic::CompareOp, exceptions::PyNotImplementedError, prelude::*};
use tantivy as tv;

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass(frozen)]
#[derive(PartialEq)]
pub(crate) struct Schema {
    pub(crate) inner: tv::schema::Schema,
}

#[pymethods]
impl Schema {
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self == other),
            CompareOp::Ne => Ok(self != other),
            _ => Err(PyNotImplementedError::new_err(format!(
                "{op:?} op is not supported"
            ))),
        }
    }
}
