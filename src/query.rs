use crate::{make_term, Schema};
use pyo3::{exceptions, prelude::*, types::PyAny};
use tantivy as tv;

/// Tantivy's Query
#[pyclass(frozen)]
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

    /// Construct a Tantivy's TermQuery
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, field_value, index_option = "position"))]
    pub(crate) fn term_query(
        schema: &Schema,
        field_name: &str,
        field_value: &PyAny,
        index_option: &str,
    ) -> PyResult<Query> {
        make_term_query(schema, field_name, field_value, index_option)
    }
}

fn make_term_query(
    schema: &Schema,
    field_name: &str,
    field_value: &PyAny,
    index_option: &str,
) -> PyResult<Query> {
    let term = make_term(&schema.inner, field_name, field_value)?;
    let index_option = match index_option {
        "position" => tv::schema::IndexRecordOption::WithFreqsAndPositions,
        "freq" => tv::schema::IndexRecordOption::WithFreqs,
        "basic" => tv::schema::IndexRecordOption::Basic,
        _ => return Err(exceptions::PyValueError::new_err(
            "Invalid index option, valid choices are: 'basic', 'freq' and 'position'"
        ))
    };
    let inner = tv::query::TermQuery::new(term, index_option);
    Ok(Query {
        inner: Box::new(inner),
    })
}
