use crate::to_pyerr;
use pyo3::{basic::CompareOp, prelude::*, types::PyTuple};
use serde::{Deserialize, Serialize};
use tantivy as tv;

/// Tantivy's Type
#[pyclass(frozen, module = "tantivy.tantivy")]
#[derive(Clone, PartialEq)]
pub(crate) enum FieldType {
    Text,
    Unsigned,
    Integer,
    Float,
    Boolean,
    Date,
    Facet,
    Bytes,
    Json,
    IpAddr,
}

impl From<FieldType> for tv::schema::Type {
    fn from(field_type: FieldType) -> tv::schema::Type {
        match field_type {
            FieldType::Text => tv::schema::Type::Str,
            FieldType::Unsigned => tv::schema::Type::U64,
            FieldType::Integer => tv::schema::Type::I64,
            FieldType::Float => tv::schema::Type::F64,
            FieldType::Boolean => tv::schema::Type::Str,
            FieldType::Date => tv::schema::Type::Date,
            FieldType::Facet => tv::schema::Type::Facet,
            FieldType::Bytes => tv::schema::Type::Bytes,
            FieldType::Json => tv::schema::Type::Json,
            FieldType::IpAddr => tv::schema::Type::IpAddr,
        }
    }
}

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass(frozen, module = "tantivy.tantivy")]
#[derive(Deserialize, PartialEq, Serialize)]
pub(crate) struct Schema {
    pub(crate) inner: tv::schema::Schema,
}

#[pymethods]
impl Schema {
    fn __richcmp__(
        &self,
        other: &Self,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[staticmethod]
    fn _internal_from_pythonized(serialized: &Bound<PyAny>) -> PyResult<Self> {
        pythonize::depythonize(&serialized).map_err(to_pyerr)
    }

    fn __reduce__<'a>(
        slf: PyRef<'a, Self>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyTuple>> {
        let serialized = pythonize::pythonize(py, &*slf).map_err(to_pyerr)?;

        Ok(PyTuple::new_bound(
            py,
            [
                slf.into_py(py).getattr(py, "_internal_from_pythonized")?,
                PyTuple::new_bound(py, [serialized]).to_object(py),
            ],
        ))
    }
}
