use crate::to_pyerr;
use pyo3::{basic::CompareOp, prelude::*, types::PyTuple};
use serde::{Deserialize, Serialize};
use tantivy as tv;

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass(frozen, module = "tantivy")]
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
    fn _internal_from_pythonized(serialized: &PyAny) -> PyResult<Self> {
        pythonize::depythonize(serialized).map_err(to_pyerr)
    }

    fn __reduce__<'a>(
        slf: PyRef<'a, Self>,
        py: Python<'a>,
    ) -> PyResult<&'a PyTuple> {
        let serialized = pythonize::pythonize(py, &*slf).map_err(to_pyerr)?;

        Ok(PyTuple::new(
            py,
            [
                slf.into_py(py).getattr(py, "_internal_from_pythonized")?,
                PyTuple::new(py, [serialized]).to_object(py),
            ],
        ))
    }
}
