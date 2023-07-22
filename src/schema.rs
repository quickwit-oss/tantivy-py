use crate::impl_py_eq;
use pyo3::prelude::*;
use tantivy as tv;

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass(module = "tantivy")]
#[derive(PartialEq)]
pub(crate) struct Schema {
    pub(crate) inner: tv::schema::Schema,
}

impl_py_eq!(Schema);

#[pymethods]
impl Schema {}
