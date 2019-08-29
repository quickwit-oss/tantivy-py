use pyo3::basic::PyObjectProtocol;
use pyo3::prelude::*;
use pyo3::types::PyType;
use tantivy::schema;
use crate::facet::Facet;
use crate::field::Field;


/// Term represents the value that the token can take.
///
/// This term can be used to access index and operate on that value
/// for example index.delete_term(term) to delete all elements with that value
#[pyclass]
pub(crate) struct Term {
    pub(crate) inner: schema::Term,
}

#[pymethods]
impl Term {
    /// Builds a term given a field, and a i64-value
    #[classmethod]
    fn from_field_i64(_cls: &PyType, field: &Field, value: i64) -> Term {
        Term {
            inner: schema::Term::from_field_i64(field.inner, value),
        }
    }

    /// Builds a term given a field, and a string value
    #[classmethod]
    fn from_field_text(_cls: &PyType, field: &Field, value: &str) -> Term {
        Term {
            inner: schema::Term::from_field_text(field.inner, value),
        }
    }

    /// Creates a `Term` given a facet.
    #[classmethod]
    fn from_facet(_cls: &PyType, field: &Field, value: &Facet) -> Term {
        Term {
            inner: schema::Term::from_facet(field.inner, &value.inner),
        }
    }

    /// Returns the field string representation.
    fn to_str(&self) -> u32 {
        self.inner.field().0
    }


}

#[pyproto]
impl PyObjectProtocol for Term {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Term({}) - {:?}", self.to_str(), self.inner.value_bytes()))
    }
}
