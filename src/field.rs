use pyo3::basic::PyObjectProtocol;
use pyo3::prelude::*;
use pyo3::types::PyType;
use tantivy::schema;
use crate::schema::Schema;

/// Field represents an element on the schema.
///
/// It can be retrieved from the schema to operate on the field.
#[pyclass]
pub(crate) struct Field {
    pub(crate) inner: schema::Field,
}

#[pymethods]
impl Field {
    /// Builds a term given a field, and a i64-value
    #[classmethod]
    fn from_schema(_cls: &PyType, schema: &Schema, field_name: &str) -> Field {
        Field {
            inner: schema.inner.get_field(field_name).unwrap()
        }
    }

    fn get_field_name(&self, schema: &Schema) -> String {
        schema.inner.get_field_name(self.inner).to_string()
    }

}

#[pyproto]
impl PyObjectProtocol for Field {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Field({})", self.inner.0))
    }
}
